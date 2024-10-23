use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

// CREATE TABLE users (
//   id SERIAL PRIMARY KEY,
//   username VARCHAR(255) NOT NULL UNIQUE,
//   email VARCHAR(255) NOT NULL UNIQUE,
//   password VARCHAR(255) NOT NULL
// );


#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserInput {
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginInput {
  pub username: String,
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
  pub id: i32,
  pub username: String,
  pub email: String,
  pub token: String,
}