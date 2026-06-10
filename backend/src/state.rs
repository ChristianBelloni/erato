use std::sync::Arc;

use axum::extract::FromRef;
use better_auth::{BetterAuth, seaorm::DatabaseConnection};

use crate::auth::AppAuthSchema;

#[derive(Clone)]
pub struct AppState {
    auth: Arc<BetterAuth<AppAuthSchema>>,
    db: DatabaseConnection,
    app_name: &'static str,
}

impl AppState {
    pub fn new(
        auth: Arc<BetterAuth<AppAuthSchema>>,
        db: DatabaseConnection,
        app_name: &'static str,
    ) -> Self {
        Self { auth, db, app_name }
    }
}

impl FromRef<AppState> for Arc<BetterAuth<AppAuthSchema>> {
    fn from_ref(state: &AppState) -> Self {
        state.auth.clone()
    }
}
