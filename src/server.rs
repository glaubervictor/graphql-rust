use crate::graphql::{MutationRoot, QueryRoot};
use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::http::{HeaderMap, Method};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Extension, Router};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use crate::auth::{context::AuthenticatedUser, jwt::decode_token};

fn create_schema(db: DatabaseConnection) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(db)
        .finish()
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn graphql_handler(
    headers: HeaderMap,
    Extension(schema): Extension<Arc<Schema<QueryRoot, MutationRoot, EmptySubscription>>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();

    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            let token = auth_str.trim_start_matches("Bearer ");
            if let Ok(claims) = decode_token(token) {
                // Insere o usuário autenticado no contexto
                request = request.data(AuthenticatedUser { claims });
            }
        }
    }

    schema.execute(request).await.into()
}

pub async fn create_app(database_url: String) -> Router {
    //database
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    //graphql schema
    let schema = Arc::new(create_schema(db.clone()));

    //cors
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    //routes
    Router::new()
        .route("/", get(graphiql))
        .route("/graphql", axum::routing::post(graphql_handler))
        .layer(Extension(schema))
        .layer(cors)
}
