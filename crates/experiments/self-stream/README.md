# Self Stream

Make a struct an actor, where it also needs to consume one of it's fields as a stream

With Actix, an `AsyncContext` has an [`add_stream`](https://docs.rs/actix/latest/actix/trait.AsyncContext.html#method.add_stream) method, but this needs to be called against a `Stream + 'static`.

Which means, we cannot simply call `add_stream` on a struct that has a field that is a `Stream`.

For example, [`libp2p::Swarm`](https://docs.rs/libp2p/latest/libp2p/struct.Swarm.html) is a `Stream`, but for a `NetworkManager` actor, it's a field of this struct.

Why a field? Because handling other methods may want to mutate the `Swarm` struct. Maybe to call [`Swarm::listen_on`](https://docs.rs/libp2p/latest/libp2p/struct.Swarm.html#method.listen_on) etc.

On closer inspection, I noticed `Actor::start` defaults to `Context::run`, which pretty much just spawns a task that drives the actor to completion.

We can replace this default implementation with our own, where we can spawn a task that drives the actor to completion, but also polls the stream, forwarding it's items to the actor.

## Case 1

In this approach, I attempt to use a channel, passing the receiving half to the actor, while we poll on the stream, and send items to the sending half.

This helps us maintain the `StreamHandler` trait, where we can implement `handle` for the actor. But also lifecycle methods like `started` and `finished`.

The downside of this approach is the actor doesn't shut down until the stream is exhausted.

## Case 2

In this approach, I attempt to break up the responsibility of receiving items from the stream, and handling them in the actor.

We're still maintaining a channel, but we poll the receiver ourselves, and send items to the actor one by one. Spawning a new task to poll the reciever again after each new one.

This way, we yield control back to the actor with no pending tasks in it's queue, at this point, if there are no strong references to `Addr`, the actor will shut down.

Using this, we cannot directly inherit `StreamHandler` behaviour via `Context::add_stream`, but I've added an adapter that calls the appropriate methods at different stages in it's lifecycle. It's debatable if we need to implement `StreamHandler` at all, `Handler` might be enough.

The downside of this approach is `spawn`-ing will allocate and pin the task on the heap. Heap allocations for every item in the stream is quite expensive.

## Case 3

In this approach, which I'd think to be the simplest and most efficient. Doesn't use channels at all, instead we make use of the `Addr` itself.

But of course if we want the actor to exit on-demand without exhausting the stream, we need only a weak reference to the `Addr`, upgrading it each time we need to message the actor.

In this approach, because we have to use the `Addr` directly, the only way to add a `StreamHandler` is to dispatch it from a `Handler` implementation, which is why we have a `FromStreamInner` enum for all the possible states of the stream.

## Case 4

This is pretty much Case 3, just that we replace the use of unsafe code with `Arc` and `Mutex`. Naturally this bumps our allocations up.

And the con of this approach is that we have to lock the mutex every time we want to interact with the swarm.

And from `Handler` / `StreamHandler` implementations, we have to return a `ResponseFuture`.

## Case 5

I noticed a race condition where mutations to the state may have been queued up before the stream is polled.

Leading to one report saying "state changed to X", followed by another saying "the state is currently X-1" from the stream.

On further investigation, I determined this was due to the stream task yielding too early, and after some iterations of more complex patterns, I settled on simply biasing the `select!` macro, which should fall back to the actor to handle the stream item before yielding, as that maintained the same behaviour.

I also swapped out `Box::pin(OptionFuture)` with a `Fuse`-d future, pinning to the same memory as the surrounding async block. This brings our allocations down, making this the most efficient approach.

## Case 6

This is pretty much Case 5, but made into a macro, to reduce boilerplate. For some unknown reason, this led to even less allocations. Yay, I guess.

## Case 6 Refined

An extension of case 6, that breaks the compatibility with other cases. For the sake of testing the ability to consume multiple streams in `Self`. This is not independently compared with the others, since it inherits the same behaviour as case 6.

## Verdict

I'd say the third and optimally, fifth (really 6th) approach is the best, I also checked allocations rudimentarily using the `alloc` module.

Contrived example, but with a stream yielding 4 items:

```console
$ cargo run -p self-stream-case1 | rg '^allocating' | awk '{sum+=$2}END{print sum}'
131939
$ cargo run -p self-stream-case2 | rg '^allocating' | awk '{sum+=$2}END{print sum}'
135323
$ cargo run -p self-stream-case3 | rg '^allocating' | awk '{sum+=$2}END{print sum}'
131227
$ cargo run -p self-stream-case4 | rg '^allocating' | awk '{sum+=$2}END{print sum}'
133464
$ cargo run -p self-stream-case5 | rg '^allocating' | awk '{sum+=$2}END{print sum}'
131050
$ cargo run -p self-stream-case6 | rg '^allocating' | awk '{sum+=$2}END{print sum}'
131031
```

```console
Case 6 < Case 5 < Case 3 < Case 1 < Case 4 < Case 2
```
