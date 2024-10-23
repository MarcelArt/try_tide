use sqlx::{Pool, Postgres};
use tide::Server;

use crate::handlers::hello_handler::hello;

use super::user_routes::setup_user_routes;

#[macro_export]
macro_rules! route_with_handler {
  ($app:expr, $method:ident, $path:expr, $handler:ident, $action:ident) => {
      {
          let handler_clone = Arc::clone(&$handler);
          $app.at($path).$method(move |req| {
              let handler = Arc::clone(&handler_clone);
              async move { handler.$action(req).await }
          });
      }
  };
}

pub fn setup_routes(app: &mut Server<()>, db: Pool<Postgres>) {
  app.at("/").post(hello);

  app.at("/api").nest({
    let mut api = tide::new();

    setup_user_routes(&mut api, db);

    api
  });
}