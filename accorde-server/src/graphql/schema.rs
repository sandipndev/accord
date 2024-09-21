use async_graphql::*;

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
    async fn dummy2(&self) -> bool {
        true
    }
}
