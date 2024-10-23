use dotenv_codegen::dotenv;
use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

pub async fn setup_postgres() -> Result<Pool<Postgres>, Error> {
  let db_user = dotenv!("DB_USER");
  let db_password = dotenv!("DB_PASSWORD");
  let db_host = dotenv!("DB_HOST");
  let db_port = dotenv!("DB_PORT");
  let db_name = dotenv!("DB_NAME");

  let db_url = format!("postgres://{}:{}@{}:{}/{}", db_user, db_password, db_host, db_port, db_name);

  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(db_url.as_str()).await?;

  println!("Connection to database success");
  Ok(pool)
}