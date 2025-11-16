use async_trait::async_trait;
use tokio::sync::mpsc;

pub struct ActorCtx<M> {
    pub myself: mpsc::Sender<M>,
}

#[async_trait]
pub trait Actor: Sized + Send + 'static {
    type Msg: Send + 'static;

    async fn handle(&mut self, msg: Self::Msg, ctx: &mut ActorCtx<Self::Msg>);
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

pub fn spawn_actor<A: Actor>(mut actor: A) -> mpsc::Sender<A::Msg> {
    let (tx, mut rx) = mpsc::channel::<A::Msg>(1024);
    let ctx = ActorCtx { myself: tx.clone() };

    tokio::spawn(async move {
        let mut ctx = ctx;
        while let Some(msg) = rx.recv().await {
            actor.handle(msg, &mut ctx).await;
            dbg!("<Actor Handle>");
        }
        dbg!("<Actor Done>");
    });

    tx
}