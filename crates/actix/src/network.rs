use actix::prelude::*;

#[derive(Default)]
pub struct NetworkManager {}

impl Actor for NetworkManager {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ListenOn {
    pub address: String,
}

impl Handler<ListenOn> for NetworkManager {
    type Result = ();

    fn handle(&mut self, _msg: ListenOn, _ctx: &mut Context<Self>) -> Self::Result {
        println!("thing");
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), BootstrapError>")]
pub struct Bootstrap {}

pub enum BootstrapError {
    Timeout,
}

impl Handler<Bootstrap> for NetworkManager {
    type Result = Result<(), BootstrapError>;

    fn handle(&mut self, _msg: Bootstrap, _ctx: &mut Context<Self>) -> Self::Result {
        todo!()
    }
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct PeerCount {
    pub topic: Option<String>,
}

impl Handler<PeerCount> for NetworkManager {
    type Result = usize;

    fn handle(&mut self, _msg: PeerCount, _ctx: &mut Context<Self>) -> Self::Result {
        todo!()
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe {
    pub topic: String,
}

enum SubscribeError {}

impl Handler<Subscribe> for NetworkManager {
    type Result = ();

    fn handle(&mut self, _msg: Subscribe, _ctx: &mut Context<Self>) -> Self::Result {
        todo!()
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Unsubscribe {
    pub topic: String,
}

impl Handler<Unsubscribe> for NetworkManager {
    type Result = ();

    fn handle(&mut self, _msg: Unsubscribe, _ctx: &mut Context<Self>) -> Self::Result {
        todo!()
    }
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Publish {
    pub topic: String,
    pub data: Vec<u8>,
}

impl Handler<Publish> for NetworkManager {
    type Result = usize;

    fn handle(&mut self, _msg: Publish, _ctx: &mut Context<Self>) -> Self::Result {
        todo!()
    }
}
