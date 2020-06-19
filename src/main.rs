use actix_files::Files;
use actix_files as fs;
use actix_web::{
    App,
    HttpServer,
    middleware,
    get,
    web
};

use openssl::ssl::{
    SslAcceptor,
    SslFiletype,
    SslMethod
};


#[get("/")]
async fn root() -> actix_web::error::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./static/root/index.html")?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("adm.web-key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("adm.web.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            // Enable the logger.
            .wrap(middleware::Logger::default())
            // We allow the visitor to see an index of the images at `/images`.

            // Keep at end
            .service(root)
            .service(Files::new("/mapping", "./static/mapping/").show_files_listing())
            .service(Files::new("/images", "./static/images/").show_files_listing())
            .service(Files::new("/public", "./static/root/").index_file("index.html"))
    })
        .bind_openssl("127.0.0.1:8088", builder)?
        .run()
        .await
}