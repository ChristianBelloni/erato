use std::sync::Arc;

use better_auth::AuthConfig;
use better_auth::AuthSchema;
use better_auth::BetterAuth;
use better_auth::email::ConsoleEmailProvider;
use better_auth::plugins::EmailPasswordPlugin;
use better_auth::plugins::EmailVerificationConfig;
use better_auth::plugins::EmailVerificationPlugin;
use better_auth::plugins::SendVerificationEmail;
use better_auth::plugins::SessionManagementPlugin;
use better_auth::seaorm::AuthEntity;
use better_auth::seaorm::SeaOrmStore;
use better_auth::seaorm::sea_orm;
use better_auth::seaorm::sea_orm::Schema;
use better_auth::seaorm::sea_orm::entity::prelude::*;

pub async fn auth(database: DatabaseConnection) -> Arc<BetterAuth<AppAuthSchema>> {
    let config = AuthConfig::new("your-very-secure-secret-key-at-least-32-chars-long")
        .trusted_origin("erato://")
        .base_url("http://localhost:3001/api/auth")
        .password_min_length(8);

    let store = SeaOrmStore::<AppAuthSchema>::new(config.clone(), database.clone());

    let verification_plugin = EmailVerificationPlugin::new()
        .require_verification_for_signin(true)
        .auto_verify_new_users(true)
        .send_on_sign_in(true);
    Arc::new(
        BetterAuth::<AppAuthSchema>::new(config)
            .email_provider(ConsoleEmailProvider)
            .store(store)
            .plugin(SessionManagementPlugin::new())
            .plugin(
                EmailPasswordPlugin::new()
                    .enable_signup(true)
                    .auto_sign_in(true)
                    .require_email_verification(true)
                    .with_email_verification(
                        EmailVerificationPlugin::new()
                            .require_verification_for_signin(true)
                            .auto_verify_new_users(true)
                            .send_on_sign_in(true)
                            .into(),
                    ),
            )
            .plugin(verification_plugin)
            .build()
            .await
            .unwrap(),
    )
}

// Only include the fields your plugins need.
// Core fields (id, name, email, etc.) are always required.
// Plugin fields (username, banned, etc.) are optional — the AuthEntity
// macro returns sensible defaults for any missing plugin field.
// Extra app-specific fields are also supported.

mod user {
    use super::*;

    #[derive(Clone, Debug, serde::Serialize, DeriveEntityModel, AuthEntity)]
    #[auth(role = "user")]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub name: Option<String>,
        pub email: Option<String>,
        pub email_verified: bool,
        pub image: Option<String>,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
        // Extra app-specific field — gets NotSet on creation,
        // populate via DB defaults or ActiveModelBehavior.
        pub locale: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

mod session {
    use super::*;

    #[derive(Clone, Debug, serde::Serialize, DeriveEntityModel, AuthEntity)]
    #[auth(role = "session")]
    #[sea_orm(table_name = "sessions")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub expires_at: DateTimeUtc,
        pub token: String,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
        pub ip_address: Option<String>,
        pub user_agent: Option<String>,
        pub user_id: String,
        pub active: bool,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

mod account {
    use super::*;

    #[derive(Clone, Debug, serde::Serialize, DeriveEntityModel, AuthEntity)]
    #[auth(role = "account")]
    #[sea_orm(table_name = "accounts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub account_id: String,
        pub provider_id: String,
        pub user_id: String,
        pub access_token: Option<String>,
        pub refresh_token: Option<String>,
        pub id_token: Option<String>,
        pub access_token_expires_at: Option<DateTimeUtc>,
        pub refresh_token_expires_at: Option<DateTimeUtc>,
        pub scope: Option<String>,
        pub password: Option<String>,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

mod verification {
    use super::*;

    #[derive(Clone, Debug, serde::Serialize, DeriveEntityModel, AuthEntity)]
    #[auth(role = "verification")]
    #[sea_orm(table_name = "verifications")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub identifier: String,
        pub value: String,
        pub expires_at: DateTimeUtc,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub struct AppAuthSchema;

impl AuthSchema for AppAuthSchema {
    type User = user::Model;
    type Session = session::Model;
    type Account = account::Model;
    type Verification = verification::Model;
}

pub async fn run_auth_migrations(database: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let schema = Schema::new(database.get_database_backend());
    for statement in [
        schema
            .create_table_from_entity(crate::auth::user::Entity)
            .if_not_exists()
            .to_owned(),
        schema
            .create_table_from_entity(session::Entity)
            .if_not_exists()
            .to_owned(),
        schema
            .create_table_from_entity(account::Entity)
            .if_not_exists()
            .to_owned(),
        schema
            .create_table_from_entity(verification::Entity)
            .if_not_exists()
            .to_owned(),
    ] {
        let _ = database.execute(&statement).await?;
    }
    Ok(())
}
