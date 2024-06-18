use std::env;
use mongodb::{Client, options::ClientOptions, Collection};
use mongodb::bson::{doc, Document};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Ballot {
    pub id: Option<bson::oid::ObjectId>,
    pub title: String,
    pub description: String,
    pub vote_count: i32,
}

impl Ballot {
    async fn get_collection() -> Collection<Ballot> {
        let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI environment variable not set!");
        let client_options = Client::parse_uri(&mongo_uri).await.expect("Failed to parse MongoDB URI");
        let client = Client::with_options(client_options).expect("Failed to initialize MongoDB client");
        client.database("DecentralizedVotingApp").collection::<Ballot>("ballots")
    }

    pub async fn create(title: String, description: String) -> mongodb::error::Result<()> {
        let collection = Self::get_collection().await;
        let new_ballot = Ballot {
            id: None,
            title,
            description,
            vote_count: 0,
        };
        collection.insert_one(new_ballot, None).await?;
        Ok(())
    }

    pub async fn retrieve_all() -> mongodb::error::Result<Vec<Ballot>> {
        let collection = Self::get_collection().await;
        let cursor = collection.find(None, None).await?;
        let ballots: Vec<Ballot> = cursor.try_collect().await?;
        Ok(ballots)
    }

    pub async fn update_details(id: bson::oid::ObjectId, title: String, description: String) -> mongodb::error::Result<()> {
        let collection = Self::get_collection().await;
        let filter = doc! { "_id": id };
        let update_doc = doc! { "$set": { "title": title, "description": description }};
        collection.update_one(filter, update_doc, None).await?;
        Ok(())
    }

    pub async fn remove(id: bson::oid::ObjectId) -> mongodb::error::Result<()> {
        let collection = Self::get_collection().await;
        collection.delete_one(doc! { "_id": id }, None).await?;
        Ok(())
    }

    pub async fn cast_vote(id: bson::oid::ObjectId) -> mongodb::error::Result<()> {
        let collection = Self::get_collection().await;
        let filter = doc! { "_id": id };
        collection.update_one(filter, doc! {"$inc": {"vote_count": 1}}, None).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    if let Ok(_) = Ballot::create("Title".into(), "Description".into()).await {
        println!("Ballot created successfully.");
    }
    if let Ok(ballots) = Ballot::retrieve_all().await {
        println!("{:?}", ballots);
    }
}