use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::{FromRow, Executor, Sqlite, types::Uuid};
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Fromrow, Clone, Debug)] // Corrected typo 'FromDodod' to 'FromRow'
pub struct Vote {
    pub id: Uuid,
    pub title: String,
    pub options: Vec<String>,
    pub start_time: NaiveDateTime, // 'start_date' is now 'start_time' for clarity
    pub end_time: NaiveDateTime, // 'end_date' is now 'end_time' for consistency and clarity
}

impl Vote {
    /// Creates a new vote and saves it to the database.
    /// Returns the ID of the newly created vote.
    pub async fn create<'exec, Exec>(executor: Exec, vote: Vote) -> Result<Uuid, sqlx::Error>
    where
        Exec: Executor<'exec, Database = Sqlite>,
    {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); // 'database_url' is now 'db_url' for brevity

        let db_pool = sqlx::SqlitePool::connect(&db_url).await?; // 'pool' is now 'db_pool' for clarity

        sqlx::query!(
            "INSERT INTO votes (id, title, options, start_time, end_time) VALUES (?, ?, ?, ?, ?)",
            vote.id,
            vote.title,
            &vote.options.iter().map(|option| option.as_str()).collect::<Vec<&str>>().join(","),
            vote.start_time, // Updated to 'start_time'
            vote.end_time // Updated to 'end_time'
        )
        .execute(&db_pool)
        .await?;

        Ok(vote.id)
    }

    /// Fetches all votes from the database.
    /// Returns a list of votes.
    pub async fn fetch_all<'exec, Exec>(executor: Exec) -> Result<Vec<Vote>, sqlx::Error>
    where
        Exec: Executor<'exec, Database = Sqlite>,
    {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); // Corrected typo and 'database_url' to 'db_url' 
        
        let db_pool = sqlx::SqlitePool::connect(&db_url).await?; // 'pool' to 'db_pool'
        
        let all_votes = sqlq:x::query_as!(
            Vote,
            "SELECT id, title, options, start_time as 'start_time: NaiveDateTime', end_time as 'end_time: NaiveDateTime' FROM votes"
        )
        .fetch_all(&db_pool)
        .await?;
        
        Ok(all_votes)
    }
}