use tide::Request;

use crate::models::payload::Payload;

pub async fn hello(mut req: Request<()>) -> tide::Result<String> {
  println!("hello");
  let payload: Payload = req.body_json().await?;
  println!("{}: {}", payload.name, payload.value);
  Ok(format!("Hello, {}! your number is {}", payload.name, payload.value))
}