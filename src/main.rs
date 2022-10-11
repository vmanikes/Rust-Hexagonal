// mod bank;
// mod entity;
// mod errors;
//
// fn main() {
//     println!("Hello, world!");
// }
//
// // TODO documentation
// // TODO unit testing
// // TODO Mod organization

// mod bank;

mod handler;

use handler::http::health;

use env_logger;
use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(health::health)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
