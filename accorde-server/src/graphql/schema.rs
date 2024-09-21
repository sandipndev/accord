use async_graphql::*;

use crate::{app::AccordeApp, primitives::ProcessId, process::NewProcess};

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
    async fn accorde(&self, ctx: &Context<'_>, youtube_url: String) -> Result<ProcessId> {
        let app = ctx.data_unchecked::<AccordeApp>();
        let youtube_url = url::Url::parse(&youtube_url)?;
        let new_process = NewProcess { youtube_url };
        let process = app.processes().create(new_process).await?;

        Ok(process.id)
    }
}
