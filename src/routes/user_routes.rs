use std::sync::Arc;

use sqlx::{Pool, Postgres};
use tide::Server;

use crate::{handlers::user_handler::UserHandler, repositories::user_repo::UserRepo, route_with_handler};

pub fn setup_user_routes(api: &mut Server<()>, db: Pool<Postgres>) {
  let user_repo = UserRepo::new(db);
  let user_handler = Arc::new(UserHandler::new(user_repo));

  api.at("/user").nest({
    let mut user = tide::new();

    route_with_handler!(user, get, "/", user_handler, read);
    route_with_handler!(user, post, "/", user_handler, create);
    route_with_handler!(user, put, "/:id", user_handler, update);
    route_with_handler!(user, delete, "/:id", user_handler, delete);
    route_with_handler!(user, get, "/:id", user_handler, get_by_id);
    route_with_handler!(user, post, "/login", user_handler, login);

    // let user_handler_clone = Arc::clone(&user_handler);
    // user.at("/").get(move |req| {
    //     let handler = Arc::clone(&user_handler_clone);
    //     async move { handler.read(req).await }
    // });

    // let user_handler_clone = Arc::clone(&user_handler);
    // user.at("/").post(move |req| {
    //     let handler = Arc::clone(&user_handler_clone);
    //     async move { handler.create(req).await }
    // });

    // let user_handler_clone = Arc::clone(&user_handler);
    // user.at("/:id").put(move |req| {
    //     let handler = Arc::clone(&user_handler_clone);
    //     async move { handler.update(req).await }
    // });

    // let user_handler_clone = Arc::clone(&user_handler);
    // user.at("/:id").delete(move |req| {
    //     let handler = Arc::clone(&user_handler_clone);
    //     async move { handler.delete(req).await }
    // });

    // let user_handler_clone = Arc::clone(&user_handler);
    // user.at("/:id").get(move |req| {
    //     let handler = Arc::clone(&user_handler_clone);
    //     async move { handler.get_by_id(req).await }
    // });

    // let user_handler_clone = Arc::clone(&user_handler);
    // user.at("/login").post(move |req| {
    //     let handler = Arc::clone(&user_handler_clone);
    //     async move { handler.login(req).await }
    // });

    user
  });
}