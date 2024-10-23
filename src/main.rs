use db::postgres::setup_postgres;
use dotenv::dotenv;
use dotenv_codegen::dotenv;
use routes::router::setup_routes;

mod routes;
mod handlers;
mod models;
mod db;
mod repositories;
mod macros;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let db = setup_postgres().await?;
    let mut app = tide::new();

    setup_routes(&mut app, db);

    let url = format!("127.0.0.1:{}", dotenv!("PORT"));
    println!("Listening on {}", url);
    app.listen(url).await?;
    Ok(())
}

// async fn hello(mut req: Request<()>) -> tide::Result<String> {
//     println!("hello");
//     let payload: Payload = req.body_json().await?;
//     println!("{}: {}", payload.name, payload.value);
//     Ok(format!("Hello, {}! your number is {}", payload.name, payload.value))
// }