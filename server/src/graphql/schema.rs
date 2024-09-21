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

    async fn get_processes(&self, ctx: &Context<'_>) -> Result<Vec<Process>> {
        let app = ctx.data_unchecked::<AccordeApp>();
        let processes = app.processes().get_all().await?;
        Ok(processes.into_iter().map(Into::into).collect())
    }
}

#[derive(Default)]
pub struct CoreMutation {}

#[Object(name = "Mutation")]
impl CoreMutation {
    async fn create_process(&self, ctx: &Context<'_>, youtube_url: String) -> Result<ProcessId> {
        let app = ctx.data_unchecked::<AccordeApp>();

        let youtube_url = url::Url::parse(&youtube_url)?;
        match youtube_url.host_str() {
            Some("www.youtube.com") | Some("youtube.com") | Some("youtu.be") => {}
            _ => return Err("Invalid youtube URL".into()),
        }

        let metadata = app.processes().get_metadata(youtube_url.as_str()).await?;

        if metadata.duration_s > 1200 {
            return Err("Video is too long".into());
        }

        let new_process = NewProcess {
            youtube_url,
            name: metadata.title,
        };
        let process = app.processes().create(new_process).await?;

        Ok(process.id)
    }
}
