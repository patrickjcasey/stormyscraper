/// This contains all of the logic needed to maintain state for stormyscraper
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    Postgres,
};

pub struct Runtime {
    db_pool: sqlx::Pool<Postgres>,
}

impl Runtime {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let rt = Self {
            db_pool: PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://user:password@localhost/postgres")
                .await?,
        };

        // perform database migrations, to ensure the database is in the expected state
        sqlx::migrate!("db/migrations").run(&rt.db_pool).await?;
        Ok(rt)
    }

    // pub update_league_games_in_db(&self, league_games: Vec<
}
