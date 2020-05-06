use actix_web::{App,HttpServer,middleware::Logger};

use log::*;
use env_logger;

mod endpoints;
use endpoints::*;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
   
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    info!("rust admin server api runing...");

    HttpServer::new(||
        App::new()
        .wrap(Logger::default())
        .service(index)
    ).bind("127.0.0.1:8080")?.run().await
}
