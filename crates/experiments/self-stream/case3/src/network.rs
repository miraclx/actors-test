use std::ops::DerefMut;
use std::pin::Pin;
use std::{ptr, task};

use actix::prelude::*;
use futures_util::future::OptionFuture;
use futures_util::stream::{StreamExt, Take};
use tokio::time;
use tokio_stream::wrappers::IntervalStream;

pub struct Swarm {
    pub value: usize,
    pub inner: Take<IntervalStream>,
}

impl Swarm {
    pub fn new(value: usize, count: usize) -> Self {
        let interval = time::interval(time::Duration::from_millis(200));

        Self {
            value,
            inner: IntervalStream::new(interval).take(count),
        }
    }
}

impl Stream for Swarm {
    type Item = usize;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        let res = Pin::new(&mut self.inner).poll_next(cx);

        res.map(|res| res.map(|_| self.value))
    }
}

impl Drop for Swarm {
    fn drop(&mut self) {
        println!("dropping the swarm");
    }
}

pub struct NetworkManager {
    pub swarm: Box<Swarm>,
}

#[derive(Message)]
#[rtype(result = "()")]
enum FromStreamInner {
    Started,
    Finished,
    Value(usize),
}

impl Handler<FromStreamInner> for NetworkManager {
    type Result = ();

    fn handle(&mut self, msg: FromStreamInner, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            FromStreamInner::Started => StreamHandler::<FromSwarm>::started(self, ctx),
            FromStreamInner::Finished => StreamHandler::<FromSwarm>::finished(self, ctx),
            FromStreamInner::Value(value) => {
                StreamHandler::<FromSwarm>::handle(self, FromSwarm { value }, ctx)
            }
        }
    }
}

impl Actor for NetworkManager {
    type Context = Context<Self>;

    fn start(mut self) -> Addr<Self> {
        let ctx = Context::new();

        // we use Box::deref_mut instead of &mut *self.swarm to avoid subtle
        // changes to the type, since this MUST be a Box, or at least a pointer
        // to a heap-allocated Swarm
        let ptr = Box::deref_mut(&mut self.swarm);

        // UNSAFE: we select! below, guaraneteeing only one use of the Swarm
        let swarm = unsafe { &mut *ptr::from_mut(ptr) };

        let addr = ctx.address();

        let mut fut = ctx.into_future(self);

        tokio::task::spawn_local({
            addr.do_send(FromStreamInner::Started);

            let addr = addr.downgrade();

            let task = async move {
                loop {
                    let item = swarm.next().await;

                    let Some(addr) = addr.upgrade() else {
                        break;
                    };

                    let Some(value) = item else {
                        addr.do_send(FromStreamInner::Finished);
                        break;
                    };

                    addr.do_send(FromStreamInner::Value(value));
                }
            };

            let mut task = Box::pin(OptionFuture::from(Some(task)));

            async move {
                loop {
                    tokio::select! {
                        _ = &mut fut => {
                            println!("network manager has shut down");
                            break;
                        },
                        Some(_) = &mut task => {
                            task = Box::pin(None.into());
                        },
                    }
                }
            }
        });

        addr
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        println!("stopping the network manager");
        Running::Stop
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FromSwarm {
    pub value: usize,
}

impl StreamHandler<FromSwarm> for NetworkManager {
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("started receiving swarm messages");
    }

    fn handle(&mut self, item: FromSwarm, _ctx: &mut Self::Context) {
        println!("got {} from the swarm", item.value);
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        println!("finished receiving swarm messages");
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ToSwarm {
    pub value: usize,
}

impl Handler<ToSwarm> for NetworkManager {
    type Result = ();

    fn handle(&mut self, msg: ToSwarm, _ctx: &mut Context<Self>) -> Self::Result {
        self.swarm.value = msg.value;
        println!("setting {} on the swarm", msg.value);
    }
}
