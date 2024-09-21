mod config;

use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{routing::get, Extension, Router};

use crate::{app::AccordeApp, graphql};

pub use config::*;

pub async fn run(config: ServerConfig, app: AccordeApp) -> anyhow::Result<()> {
    let schema = graphql::schema(Some(app));

    let app = Router::new()
        .route(
            "/graphql",
            get(playground).post(axum::routing::post(graphql_handler)),
        )
        .layer(Extension(schema));

    println!("Starting graphql server on port {}", config.port);
    let listener =
        tokio::net::TcpListener::bind(&std::net::SocketAddr::from(([0, 0, 0, 0], config.port)))
            .await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

pub async fn graphql_handler(
    schema: Extension<Schema<graphql::CoreQuery, graphql::CoreMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner();
    let res = schema.execute(req).await;
    res.into()
}

async fn playground() -> impl axum::response::IntoResponse {
    axum::response::Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}
