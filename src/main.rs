use axum::{
    routing::{post},
    Router,   
    extract::Json
};

use serde::{Serialize,Deserialize};
// use serde_json::{Value, json};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};

#[derive(Deserialize)]
struct UserDetails {
    username: String,
    forename: String,
    middlename: Option<String>,
    surname: String,
    age: u8
}

#[derive(Serialize,Deserialize)]
struct ApiResponse {
    success: bool,
    message: String
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/",post(route_handler));    

    //run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
       .serve(app.into_make_service())
       .await
       .unwrap();
}

async fn route_handler(Json(payload): Json<UserDetails>) -> Json<ApiResponse>{
    
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let middle_name: String = match payload.middlename {
        Some(x) => format!(" {} ",x),
        None => " ".to_string()
    };
    let message = format!("User {}{}{} registered successfully. He is {} years old.",payload.forename,middle_name,payload.surname,payload.age);
    let response: ApiResponse = ApiResponse {success: true, message: message};
    Json(response)
}
