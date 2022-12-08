mod config;

use std::sync::Mutex;
use actix_web::{App, HttpServer, web, get};
use crate::config::config::Counter;
use crate::config::config::index_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //state best buiten de http thread zodat die globaal gedeeld kan worden
    //omdat er telkens een nieuwe instantie van httpserver wordt gemaakt per request
    let counter = web::Data::new(Counter::new(0));

    HttpServer::new(move || {
        App::new()
            .configure(index_config)
            .app_data(counter.clone())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
