use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::{FromRow, Executor, Sqlite, types::Uuid};
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)] 
pub struct VotingEvent {
    pub id: Uuid,
    pub topic: String,
    pub choices: Vec<String>,
    pub commencement: NaiveDateTime,
    pub conclusion: NaiveDateTime,
}

impl VotingEvent {
    pub async fn save_new<'exec, E>(executor: E, event: VotingEvent) -> Result<Uuid, sqlx::Error>
    where
        E: Executor<'exec, Database = Sqlite>,
    {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection_pool = sqlx::SqlitePool::connect(&database_url).await?;
        
        let options_str = event.choices.join(",");

        sqlx::query!(
            "INSERT INTO voting_events (id, topic, choices, commencement, conclusion) VALUES (?, ?, ?, ?, ?)",
            event.id,
            event.topic,
            &options_str,
            event.commencement,
            event.conclusion
        )
        .execute(&connection_pool)
        .await?;

        Ok(event.id)
    }

    pub async fn retrieve_all<'exec, E>(executor: E) -> Result<Vec<VotingEvent>, sqlx::Error>
    where
        E: Executor<'exec, Database = Sqlite>,
    {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_FORMAT must be set");
        let connection_pool = sqlx::SqlitePool::connect(&database_url).await?;
        
        let voting_events = sqlx::query_as!(
            VotingEvent,
            "SELECT id, topic, choices, commencement as 'commencement: NaiveDateTime', conclusion as 'conclusion: NaiveDateTime' FROM voting_events"
        )
        .fetch_all(&connection_pool)
        .await?;
        
        Ok(voting_events)
    }
}