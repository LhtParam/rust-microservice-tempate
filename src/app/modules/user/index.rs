use actix_web::web;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::extjson::de::Error;
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::Client;

#[path = "../../constants/index.rs"]
mod constants;
#[path = "../../models/user.rs"]
pub(crate) mod model;

use model::User;

pub async fn create_user(client: web::Data<Client>,request_data: User,) -> Result<InsertOneResult, Error> {
    let user_data = User {
        id: request_data.id,
        first_name: request_data.first_name,
        last_name: request_data.last_name,
        username: request_data.username,
        email: request_data.email,
    };

    let user = model::insert_one(client, user_data).await;

    Ok(user)
}

pub async fn get_user(client: web::Data<Client>, id: web::Path<String>) -> Result<User, Error> {
    let _id = id.into_inner();

    let user_detail = model::find_one(client, _id).await;

    Ok(user_detail.unwrap())
}

pub async fn update_user(
    client: web::Data<Client>,
    user_id: String,
    request_data: User,
    uid: String,
) -> Result<User, Error> {
    let update_id = user_id;
    let filter = doc! {"id": update_id};

    let new_doc = doc! {
        "$set": {
            "id": request_data.id,
            "first_name": request_data.first_name,
            "last_name": request_data.last_name,
            "username": request_data.username,
            "email": request_data.email,
        }
    };

    let update_response = model::update_one(client, filter, new_doc, uid).await;

    Ok(update_response.unwrap())
}

pub async fn get_all_users(client: web::Data<Client>) -> Result<Vec<User>, Error> {
    let mut users: Vec<User> = Vec::new();

    let mut result = model::find(client).await;

    while let Some(user) = result
        .try_next()
        .await
        .ok()
        .expect("Error mapping through result")
    {
        users.push(user)
    }

    Ok(users)
}

pub async fn delete_user(client: web::Data<Client>, id: String) -> Result<DeleteResult, Error> {
    let filter = doc! {"id": id};

    let delete_response = model::delete_one(client, filter).await;

    Ok(delete_response)
}
