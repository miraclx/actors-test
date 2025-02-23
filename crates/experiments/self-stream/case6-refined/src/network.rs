use std::ops::RangeFrom;

use actix::prelude::*;
use futures_util::stream::{repeat, Iter, Repeat};

use crate::actor_stream::spawn_actor;
use crate::stream_gen::StreamGen;

type RepeatingStream = StreamGen<Repeat<usize>, usize>;
type CountingStream = StreamGen<Iter<RangeFrom<usize>>, FromCounter>;

pub struct NetworkManager {
    pub repeat: Box<RepeatingStream>,
    pub counter: Box<CountingStream>,
}

impl Actor for NetworkManager {
    type Context = Context<Self>;

    fn start(mut self) -> Addr<Self> {
        spawn_actor!(
            self @ NetworkManager => {
                .repeat as FromRepeatingStream,
                .counter,
            }
        )
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        println!("stopping the network manager");
        Running::Stop
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FromRepeatingStream {
    pub value: usize,
}

impl From<usize> for FromRepeatingStream {
    fn from(value: usize) -> Self {
        Self { value }
    }
}

impl StreamHandler<FromRepeatingStream> for NetworkManager {
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("started receiving repeated messages");
    }

    fn handle(&mut self, item: FromRepeatingStream, _ctx: &mut Self::Context) {
        println!("got {} from the repeating stream", item.value);
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        println!("finished receiving repeated messages");
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FromCounter {
    pub value: usize,
}

impl From<usize> for FromCounter {
    fn from(value: usize) -> Self {
        Self { value }
    }
}

impl StreamHandler<FromCounter> for NetworkManager {
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("started receiving counter messages");
    }

    fn handle(&mut self, item: FromCounter, _ctx: &mut Self::Context) {
        println!("got {} from the counter", item.value);
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        println!("finished receiving counter messages");
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct UpdateRepeatingStreamValue {
    pub value: usize,
}

impl Handler<UpdateRepeatingStreamValue> for NetworkManager {
    type Result = ();

    fn handle(
        &mut self,
        msg: UpdateRepeatingStreamValue,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        *self.repeat.stream_mut() = repeat(msg.value);

        println!("setting {} on the repeating stream", msg.value);
    }
}
