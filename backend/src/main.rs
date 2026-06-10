use axum::Router;
use better_auth::integrations::axum::AxumIntegration;
use better_auth::seaorm::Database;
use erato::auth::{auth, run_auth_migrations};
use erato::state::AppState;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://better-auth-axum.db?mode=rwc".to_string());
    let database = Database::connect(&database_url).await?;
    run_auth_migrations(&database).await?;

    let auth = auth(database.clone()).await;

    let state = AppState::new(auth.clone(), database, "erato");

    let app = Router::new()
        .nest("/api/auth", auth.axum_router_with_state::<AppState>())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
