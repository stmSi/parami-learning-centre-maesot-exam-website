pub mod exam;

use axum::routing::Router;
use tower_http::services::ServeDir;

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/exam", exam::routes())
        .nest_service("/assets", ServeDir::new("assets"))
}
