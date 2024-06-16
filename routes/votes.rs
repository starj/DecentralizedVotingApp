use std::env;
use mongodb::{Client, options::ClientOptions, Collection};
use mongodb::bson::{doc, Document};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Vote {
    pub id: Option<bson::oid::ObjectId>,
    pub title: String,
    pub description: String,
    pub votes: i32,
}

impl Vote {
    async fn initialize_collection() -> Collection<Vote> {
        let mongo_uri = env::var("MONGO_URI").expect("You must set the MONGO_URI environment variable!");
        let client_options = ClientOptions::parse(&mongo_uri).await.expect("Failed to parse options");
        let client = Client::with_options(client_options).expect("Failed to initialize client");
        client.database("DecentralizedVotingApp").collection::<Vote>("votes")
    }

    pub async fn create_new_vote(title: String, description: String) -> mongodb::error::Result<()> {
        let collection = Self::initialize_collection().await;
        let new_vote = Vote {
            id: None,
            title,
            description,
            votes: 0,
        };
        collection.insert_one(new_vote, None).await?;
        Ok(())
    }

    pub async fn get_votes() -> mongodb::error::Result<Vec<Vote>> {
        let collection = Self::initialize_collection().await;
        let cursor = collection.find(None, None).await?;
        let votes: Vec<Vote> = cursor.try_collect().await?;
        Ok(votes)
    }

    pub async fn update_vote_details(id: bson::oid::ObjectId, title: String, description: String) -> mongodb::error::Result<()> {
        let collection = Self::initialize_collection().await;
        let filter = doc! { "_id": id };
        let update_doc = doc! { "$set": { "title": title, "description": description }};
        collection.update_one(filter, update_doc, None).await?;
        Ok(())
    }

    pub async fn delete_vote(id: bson::oid::ObjectId) -> mongodb::error::Result<()> {
        let collection = Self::initialize_collection().await;
        collection.delete_one(doc! { "_id": id }, None).await?;
        Ok(())
    }

    pub async fn vote(id: bson::oid::ObjectId) -> mongodb::error::Result<()> {
        let collection = Self::initialize_collection().await;
        let filter = doc! { "_id": id };
        collection.update_one(filter, doc! {"$inc": {"votes": 1}}, None).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Example usage
    // Remember to handle results in real use cases
    if let Ok(_) = Vote::create_new_vote("Title".into(), "Description".into()).await {
        println!("Vote created successfully.");
    }
    if let Ok(votes) = Vote::get_votes().await {
        println!("{:?}", votes);
    }
    // Place here the functionality to update, delete, and increment vote counts as needed.
}