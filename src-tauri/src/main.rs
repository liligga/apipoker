// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::collections::HashMap;

use serde;



#[derive(Debug, serde::Deserialize)]
struct UrlParams {
    params: HashMap<String, String>,
}


#[derive(Debug, serde::Deserialize)]
struct RequestBody {
    request_body_type: String,
    request_body_contents: String,
}

#[derive(Debug, serde::Deserialize)]
struct RequestHeaders {
    headers: HashMap<String, String>,
}


#[derive(Debug, serde::Deserialize)]
enum RequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}


#[derive(Debug, serde::Deserialize)]
struct Request {
    url: String,
    params: UrlParams,
    method: RequestMethod,
    body: RequestBody,
    headers: RequestHeaders,
}

#[derive(Debug)]
struct Response {
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
    cookies: HashMap<String, String>,
}


#[derive(Debug)]
struct RequestHistory {
    items: Vec<HashMap<Request, Response>>,
}   

#[tauri::command]
fn make_request(request: Request) -> String {
    println!("This is the method for making requests, {request:?}");
    format!("This is the method for making requests, {request:?}")
}

#[tauri::command]
fn save_request(request: Request) -> String {
    println!("This is the method for saving requests, {request:?}");
    format!("This is the method for saving requests, {request:?}")
}

#[tauri::command]
fn save_response(request: Request, response: Response) {
    println!("This is the method for saving responses, {request:?}, {response:?}");
    format!("This is the method for saving responses, {request:?}, {response:?}")
}


#[tauri::command]
fn load_request_history(url: &str) -> String {
    println!("This is the method for loading request history, {url:?}");
    format!("This is the method for loading request history, {url:?}")
}

#[tauri::command]
fn create_collection(name: &str) -> String {
    println!("This is the method for creating collections, {name:?}");
    format!("This is the method for creating collections, {name:?}")
}

#[tauri::command]
fn delete_collection(name: &str) -> String {
    println!("This is the method for deleting a collection, {name:?}");
    format!("This is the method for deleting a collection, {name:?}")
}

#[tauri::command]
fn update_collection(name: &str) -> String {
    println!("This is the method for updating a collection, {name:?}");
    format!("This is the method for updating a collection, {name:?}")
}

#[tauri::command]
fn add_request_to_collection(name: &str, request: Request) -> String {
    println!("This is the method for adding a request to a collection, {name:?}, {request:?}");
    format!("This is the method for adding a request to a collection, {name:?}, {request:?}")
}

#[tauri::command]
fn get_collection(name: &str) -> String {
    println!("This is the method for getting a collection from DB, {name:?}");
    format!("This is the method for getting a collection from DB, {name:?}")
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            make_request, 
            // save_request, 
            // save_response, 
            // load_request_history, 
            // create_collection, 
            // delete_collection, 
            // update_collection,
            // get_collection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
