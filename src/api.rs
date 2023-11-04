use axum::{extract::Path, http::StatusCode, Json};
use bson::oid::ObjectId;

use crate::{db::Db, room::Room};

pub async fn get_rooms() -> Result<Json<Vec<Room>>, StatusCode> {
    let db = Db::new().await;
    let rooms = db.get_rooms().await.map_err(|e| {
        println!("{e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(rooms))
}

pub async fn get_room(Path(room_id): Path<ObjectId>) -> Result<Json<Room>, StatusCode> {
    let db = Db::new().await;
    let room = db
        .get_room(&room_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(room))
}
