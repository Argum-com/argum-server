use bson::{doc, oid::ObjectId};
use futures_util::TryStreamExt;
use mongodb::{error::Error, Client};

use crate::room::Room;

pub struct Db {
    client: Client,
}

impl Db {
    pub async fn new() -> Self {
        let client = Client::with_uri_str(std::env::var("URI").unwrap())
            .await
            .expect("Failed to connect to MongoDB");
        Self { client }
    }
    pub async fn get_rooms(self) -> Result<Vec<Room>, Error> {
        self.client
            .database("dev")
            .collection("rooms")
            .find(None, None)
            .await?
            .try_collect::<Vec<Room>>()
            .await
    }
    pub async fn get_room(self, room_id: &ObjectId) -> Result<Room, Error> {
        self.client
            .database("dev")
            .collection("rooms")
            .find_one(
                Some(doc! {
                    "_id": room_id
                }),
                None,
            )
            .await?
            .ok_or(Error::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Room not found",
            )))
    }
}
