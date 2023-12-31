use gram_job_board::{AppState, BackgroundServices};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use std::{process, sync::Arc};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> Result<BackgroundServices, shuttle_runtime::Error> {
    sqlx::migrate!().run(&pool).await.unwrap_or_else(|err| {
        eprintln!("Unable to migrate sql files: {err}");
        process::exit(1);
    });

    let teloxide_token = secrets.get("TELOXIDE_TOKEN").unwrap();
    let state = Arc::new(AppState::new(pool, teloxide_token));

    let router = gram_job_board::create_router(Arc::clone(&state));

    Ok(BackgroundServices::new(state, router))
}
