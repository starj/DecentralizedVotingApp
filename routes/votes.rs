use mongodb::{Client, options::ClientOptions, Collection};
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::error::Error as MongoError;
use serde::{Serialize, Deserialize};
use std::{env, error::Error};
use futures::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]
struct Ballot {
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub vote_count: i32,
}

impl Ballot {
    async fn get_collection() -> Result<Collection<Ballot>, Box<dyn Error>> {
        let mongo_uri = env::var("MONGO_URI").map_err(|_| "MONGO_URI environment variable not set!")?;
        let client_options = Client::parse_uri(&mongo_uri).await.map_err(|_| "Failed to parse MongoDB URI")?;
        let client = Client::with_options(client_options).map_err(|_| "Failed to initialize MongoDB client")?;
        Ok(client.database("DecentralizedVotingApp").collection::<Ballot>("ballots"))
    }

    pub async fn create(title: String, description: String) -> Result<(), Box<dyn Error>> {
        let collection = Self::get_collection().await?;
        let new_ballot = Ballot {
            id: None,
            title,
            description,
            vote_count: 0,
        };
        collection.insert_one(new_ballot, None).await.map_err(|e| e.into())
    }

    pub async fn retrieve_all() -> Result<Vec<Ballot>, Box<dyn Error>> {
        let collection = Self::get_collection().await?;
        let cursor = collection.find(None, None).await.map_err(|e| e.into())?;
        let ballots: Vec<Ballot> = cursor.try_collect().await.map_err(|e| e.into())?;
        Ok(ballots)
    }

    pub async fn update_details(id: ObjectId, title: String, description: String) -> Result<(), Box<dyn Error>> {
        let collection = Self::get_collection().await?;
        let filter = doc! { "_id": id };
        let update_doc = doc! { "$set": { "title": title, "description": description }};
        collection.update_one(filter, update_doc, None).await.map_err(|e| e.into())
    }

    pub async fn remove(id: ObjectId) -> Result<(), Box<dyn Error>> {
        let collection = Self::get_collection().await?;
        collection.delete_one(doc! { "_id": id }, None).await.map_err(|e| e.into())
    }

    pub async fn cast_vote(id: ObjectId) -> Result<(), Box<dyn Error>> {
        let collection = Self::get_collection().await?;
        let filter = doc! { "_id": id };
        collection.update_one(filter, doc! {"$inc": {"vote_count": 1}}, None).await.map_path(|e| e.into())
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    match Ballot::create("Title".into(), "Description".into()).await {
        Ok(_) => println!("Ballot created successfully."),
        Err(e) => println!("Failed to create ballot: {}", e),
    }
    match Ballot::retrieve_all().await {
        Ok(ballots) => println!("{:?}", ballots),
        Err(e) => println!("Failed to retrieve ballots: {}", e),
    }
}