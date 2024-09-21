use async_graphql::*;

use crate::{app::AccordeApp, primitives::ProcessId, process::NewProcess};

use super::process::Process;

#[derive(Default)]
pub struct CoreQuery {}

#[Object(name = "Query")]
impl CoreQuery {
    async fn get_process(&self, ctx: &Context<'_>, id: ProcessId) -> Result<Process> {
        let app = ctx.data_unchecked::<AccordeApp>();
        let process = app.processes().get(id).await?;
        Ok(process.into())
    }
}

#[derive(Default)]
pub struct CoreMutation {}

#[Object(name = "Mutation")]
impl CoreMutation {
    async fn create_process(&self, ctx: &Context<'_>, youtube_url: String) -> Result<ProcessId> {
        let app = ctx.data_unchecked::<AccordeApp>();
        let youtube_url = url::Url::parse(&youtube_url)?;
        let new_process = NewProcess { youtube_url };
        let process = app.processes().create(new_process).await?;

        Ok(process.id)
    }
}
