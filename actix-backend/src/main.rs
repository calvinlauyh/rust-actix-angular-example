use std::io;
use actix_files;
use actix_web::http::StatusCode;
use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer, Result};
use log::info;

fn rust_index() -> Result<HttpResponse> {
    info!("Simple web page served by Rust");
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(format!("Hello from Rust")))
}

fn angular_index() -> Result<actix_files::NamedFile> {
    info!("Redirected to Angular SPA");
    Ok(actix_files::NamedFile::open("../angular-ui/dist/index.html")?)
}

fn main() -> io::Result<()> {
    env_logger::init();

    HttpServer::new( || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/rust").route(web::get().to(rust_index)))
            .service(actix_files::Files::new("/static", "../angular-ui/dist"))
            .default_service(
                web::resource("")
                    .route(web::get().to(angular_index))
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed()),
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
