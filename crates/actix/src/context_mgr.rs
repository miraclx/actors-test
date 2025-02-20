use actix::prelude::*;

// mod listen;
// mod subscribe;
// mod unsubscribe;

pub struct ContextManager {}

impl Actor for ContextManager {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype("T::Result")]
struct Delayed<T: Message> {
    pub msg: T,
    pub delay: std::time::Duration,
}

#[derive(Message)]
#[rtype("eyre::Result<()>")]
struct CreateContext {
    pub protocol: String,
    pub seed: Option<[u8; 32]>,
    pub application_id: String,
    pub identity_secret: Option<String>,
    pub initialization_params: Vec<u8>,
}

impl Handler<CreateContext> for ContextManager {
    type Result = eyre::Result<()>;

    fn handle(&mut self, _msg: CreateContext, _ctx: &mut Context<Self>) -> Self::Result {
        todo!()
    }
}

#[derive(Message)]
#[rtype("()")]
struct JoinContext {
    pub identity_secret: String,
    pub invitation_payload: String,
}

impl Handler<JoinContext> for ContextManager {
    type Result = ();

    fn handle(&mut self, _msg: JoinContext, _ctx: &mut Context<Self>) -> Self::Result {
        todo!()
    }
}

#[derive(Message)]
#[rtype("()")]
struct Invite {
    pub context_id: String,
    pub inviter_id: String,
    pub invitee_id: String,
}

impl Handler<Invite> for ContextManager {
    type Result = ();

    fn handle(&mut self, _msg: Invite, _ctx: &mut Context<Self>) -> Self::Result {
        todo!()
    }
}
