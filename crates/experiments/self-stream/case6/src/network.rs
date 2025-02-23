use actix::prelude::*;
use futures_util::stream::{repeat, Repeat};

use crate::actor_stream::spawn_actor;
use crate::stream_gen::StreamGen;

type RepeatingStream = StreamGen<Repeat<usize>, usize>;

pub struct NetworkManager {
    pub repeat: Box<RepeatingStream>,
}

impl Actor for NetworkManager {
    type Context = Context<Self>;

    fn start(mut self) -> Addr<Self>
    where
        Self: Actor<Context = Context<Self>>,
    {
        spawn_actor!(
            self @ NetworkManager => {
                .repeat as FromSwarm,
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
pub struct FromSwarm {
    pub value: usize,
}

impl From<usize> for FromSwarm {
    fn from(value: usize) -> Self {
        Self { value }
    }
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
        *self.repeat.stream_mut() = repeat(msg.value);

        println!("setting {} on the swarm", msg.value);
    }
}
