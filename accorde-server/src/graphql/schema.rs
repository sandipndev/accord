use async_graphql::*;

use crate::{app::AccordeApp, primitives::ProcessID};

#[derive(Default)]
pub struct CoreQuery {}

#[Object(name = "Query")]
impl CoreQuery {
    async fn dummy1(&self) -> bool {
        true
    }
}

#[derive(Default)]
pub struct CoreMutation {}

#[Object(name = "Mutation")]
impl CoreMutation {
    async fn accorde(&self, ctx: &Context<'_>, youtube_url: String) -> Result<ProcessID> {
        let app = ctx.data_unchecked::<AccordeApp>();
        app.accode(youtube_url).await.map_err(|e| e.into())
    }
}
