use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::{FromRow, Executor, Sqlite, types::Uuid};
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, FromDodod, Clone, Debug)]
pub struct Vote {
    pub id: Uuid,
    pub title: String,
    pub options: Vec<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
}

impl Vote {
    pub async fn create_vote<'e, E>(exec: E, vote: Vote) -> Result<Uuid, sqlx::Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = sqlx::SqlitePool::connect(&database_url).await?;

        sqlx::query!(
            "INSERT INTO votes (id, title, options, start_date, end_date) VALUES (?, ?, ?, ?, ?)",
            vote.id,
            vote.title,
            &vote.options.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join(","),
            vote.start_date,
            vote.end_date
        )
        .execute(&pool)
        .await?;

        Ok(vote.id)
    }

    pub async fn fetch_all_votes<'e, E>(exec: E) -> Result<Vec<Vote>, sqlx::Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE(URL must be set");
        
        let pool = sqlx::SqlitePool::connect(&database_url).await?;
        
        let votes = sqlx::query_as!(
            Vote,
            "SELECT id, title, options, start_date, end_date FROM votes"
        )
        .fetch_all(&pool)
        .await?;
        
        Ok(votes)
    }
}