mod schema;
pub use schema::*;

use async_graphql::*;

use crate::app::AccordeApp;

pub fn schema(app: Option<AccordeApp>) -> Schema<CoreQuery, CoreMutation, EmptySubscription> {
    let schema = Schema::build(
        CoreQuery::default(),
        CoreMutation::default(),
        EmptySubscription,
    );
    if let Some(app) = app {
        schema.data(app).finish()
    } else {
        schema.finish()
    }
}
