use actix::prelude::*;

use super::Network;

#[derive(Message)]
#[rtype(result = "usize")]
struct ListenOn {}

impl Handler<ListenOn> for Network {
    type Result = ResponseFuture<usize>;

    fn handle(&mut self, _msg: ListenOn, _ctx: &mut Context<Self>) -> Self::Result {
        todo!()
        // let act_addr = AnswerActor::from_registry();
        // let request = act_addr.send(ListenOn {});
        // Box::pin(async move { request.await.unwrap() })
    }
}
