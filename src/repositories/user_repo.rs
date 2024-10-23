use sqlx::{Error, Pool, Postgres};

use crate::models::user::{User, UserInput};

pub struct UserRepo {
  db: Pool<Postgres>
}

impl UserRepo {
  pub fn new(db: Pool<Postgres>) -> UserRepo {
    UserRepo { db }
  }

  pub async fn create(&self, user: UserInput) -> Result<i32, Error> {
    let result: (i32, ) = sqlx::query_as("INSERT INTO users (username, email, password) values ($1, $2, $3) RETURNING id")
      .bind(user.username)
      .bind(user.email)
      .bind(user.password)
      .fetch_one(&self.db)
      .await?;
    
    let id = result.0;
    Ok(id)
  }

  pub async fn read(&self) -> Result<Vec<User>, Error> {
    let users: Vec<User> = sqlx::query_as::<_, User>( "select * from users")
      .fetch_all(&self.db)
      .await?;

    Ok(users)
  }

  pub async fn update(&self, id: i32, user: UserInput) -> Result<String, Error> {
    let query = "
      update users
      set 
        username = $1,
        email = $2
      where id = $3
    ";

    sqlx::query(query)
      .bind(user.username)
      .bind(user.email)
      .bind(id)
      .execute(&self.db)
      .await?;

    Ok(String::from("updated"))
  }

  pub async fn delete(&self, id: i32) -> Result<String, Error> {
    let query = "delete from users where id = $1";

    sqlx::query(query)
      .bind(id)
      .execute(&self.db)
      .await?;

    Ok(String::from("deleted"))
  }

  pub async fn get_by_id(&self, id: i32) -> Result<User, Error> {
    let query = "select * from users where id = $1";

    let user = sqlx::query_as::<_, User>(query)
      .bind(id)
      .fetch_one(&self.db)
      .await?;

    Ok(user)
  }

  pub async fn get_by_username(&self, username: String) -> Result<Option<User>, Error> {
    let query = "select * from users where username = $1";

    let user = sqlx::query_as::<_, User>(query)
      .bind(username)
      .fetch_optional(&self.db)
      .await?;

    Ok(user)
  }
}