use async_graphql::*;

use crate::{
    app::AccordeApp,
    primitives::{TrackId, YoutubeUrl},
    tracks::NewTrack,
};

use super::track::Track;

#[derive(Default)]
pub struct CoreQuery {}

#[Object(name = "Query")]
impl CoreQuery {
    async fn track(&self, ctx: &Context<'_>, track_id: TrackId) -> Result<Track> {
        let app = ctx.data_unchecked::<AccordeApp>();
        let track = app.tracks().get_by_id(track_id).await?;
        Ok(track.into())
    }

    async fn tracks(&self, ctx: &Context<'_>) -> Result<Vec<Track>> {
        let app = ctx.data_unchecked::<AccordeApp>();
        let tracks = app.tracks().get_all().await?;
        Ok(tracks.into_iter().map(Into::into).collect())
    }
}

#[derive(Default)]
pub struct CoreMutation {}

#[Object(name = "Mutation")]
impl CoreMutation {
    async fn create_track(&self, ctx: &Context<'_>, youtube_url: YoutubeUrl) -> Result<Track> {
        let app = ctx.data_unchecked::<AccordeApp>();

        let new_track = NewTrack { youtube_url };
        let track = app.tracks().create(new_track).await?;

        Ok(track.into())
    }
}
