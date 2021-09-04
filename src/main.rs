use std::str::FromStr;

use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware::Logger;
use anyhow::Context;

mod frontend;
mod backend;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    if let Err(std::env::VarError::NotPresent) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let bind = match std::env::var("WATER_LEVELS_HOST") {
        Ok(var) => var,
        Err(e) => anyhow::bail!("WATER_LEVELS_HOST not available: {:?}", e),
    };

    let port = match std::env::var("WATER_LEVELS_PORT") {
        Ok(var) => u16::from_str(&var).context("Parsing port to u16")?,
        Err(e) => anyhow::bail!("WATER_LEVELS_PORT not available: {:?}", e),
    };

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(crate::frontend::css)
            .service(crate::frontend::index)
            .service(crate::frontend::make_landscape)
            .service(crate::frontend::calculate)
    })
    .bind((bind, port))?
    .run()
    .await
    .map_err(anyhow::Error::from)
}
