use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware::Logger;

mod frontend;
mod backend;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(crate::frontend::index)
            .service(crate::frontend::make_landscape)
            .service(crate::frontend::calculate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
