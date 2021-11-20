use crate::dao::*;
use crate::cache::Configuration;
use nanoid::nanoid;
use serde::*;
use serde_json::json;


#[derive(Serialize, Deserialize)]
pub struct PostResponse {
    inserted: bool,
    username: String,
}

// new user
pub async fn post_data(
    username: String,
    password: String,
    info: serde_json::Value) -> PostResponse {
    let id = nanoid!();
    let key_with_max_ver: String = format!("/user/?username={}&password={}", username, password);
    let res: serde_json::Value = json!({
        "_id": id,
        "username": username,
        "password": password,
        "info": info,
    });
    configuration::create_user(&key_with_max_ver, &res);
    let resp = PostResponse {
        inserted: true,
        username: username,
    };
    resp
}

// delete user
// pub async fn delete_data(key: &String) -> PostResponse {
//     let response = configuration::delete_user(&key);
//     resp
// }

//get user
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    service_name: Option<String>,
    enviroment: Option<String>,
    version: Option<u32>,
}
pub async fn get_all(key: &String) -> serde_json::Value {
    if Configuration::exists_in_cache(&key) {
        let conf = Configuration::get_from_cache(&key);
        conf
    } else {
        let conf = configuration::get_user(&key);
        Configuration::store_in_cache(&key, &conf);
        conf
    }
}
//update user

#[derive(Serialize, Deserialize)]
pub struct UpdateResponse {
    updated_username: String,
    updated_password: String,
    updated_info: serde_json::Value,
}
pub async fn put_data(
    doc_to_update: &(String, String),
    updated_doc: &(String, String, serde_json::Value),
) -> UpdateResponse {
    let key_to_update: String = format!(
        "/user/?username={}&password={}",
        doc_to_update.0, doc_to_update.1
    );
    let updated_key: String = format!(
        "/user/?username={}&password={}",
        updated_doc.0, updated_doc.1
    );
    let res: serde_json::Value = json!({
        "username": updated_doc.0,
        "password": updated_doc.1,
        "info": updated_doc.2,
    });
    configuration::update_user(&key_to_update, &updated_key, &res);
    let response = UpdateResponse {
        updated_username: updated_doc.0.to_owned(),
        updated_password: updated_doc.1.to_owned(),
        updated_info: updated_doc.2.to_owned(),
    };
    response
}
