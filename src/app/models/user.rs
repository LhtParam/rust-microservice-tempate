#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused)]

use crate::constants;
use actix_web::web;
use dotenv::dotenv;
use mongodb::bson::{doc, Document};
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::{Client, Collection, Cursor};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}

pub fn get_db_details() -> (String, String) {
    let db_name = match env::var(constants::DB_NAME) {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable DB_NAME"),
    };
    let user_collection = match env::var(constants::USER_COLLECTION) {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable USER_COLLECTION"),
    };
    return (db_name, user_collection);
}

pub async fn insert_one(client: web::Data<Client>, user_data: User) -> InsertOneResult {
    dotenv().ok();

    let (db_name, user_collection) = get_db_details();

    let collection: Collection<User> = client.database(&db_name).collection(&user_collection);

    let response = collection
        .insert_one(user_data, None)
        .await
        .ok()
        .expect("Error creating user");

    response
}

pub async fn find_one(client: web::Data<Client>, id: String) -> Option<User> {
    let (db_name, user_collection) = get_db_details();

    let collection: Collection<User> = client.database(&db_name).collection(&user_collection);

    let user_details = collection
        .find_one(doc! {"id": id}, None)
        .await
        .ok()
        .expect("Error getting user's detail");

    user_details
}

pub async fn update_one(
    client: web::Data<Client>,
    filter: Document,
    new_doc: Document,
    uid: String,
) -> Option<User> {
    let (db_name, user_collection) = get_db_details();

    let collection: Collection<User> = client.database(&db_name).collection(&user_collection);

    collection
        .update_one(filter, new_doc, None)
        .await
        .ok()
        .expect("Error updating user");

    let response = find_one(client, uid).await;

    response
}

pub async fn find(client: web::Data<Client>) -> Cursor<User> {
    let (db_name, user_collection) = get_db_details();

    let collection: Collection<User> = client.database(&db_name).collection(&user_collection);

    let response = collection
        .find(None, None)
        .await
        .ok()
        .expect("Error fetching user details");

    response
}

pub async fn delete_one(client: web::Data<Client>, filter: Document) -> DeleteResult {
    let (db_name, user_collection) = get_db_details();

    let collection: Collection<User> = client.database(&db_name).collection(&user_collection);

    let response = collection
        .delete_one(filter, None)
        .await
        .ok()
        .expect("Error deleting user");

    response
}
