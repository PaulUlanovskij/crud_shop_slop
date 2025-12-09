use std::{net::TcpListener};

use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpServer, dev::Server, web};
use sea_orm::DatabaseConnection;

pub mod config;
pub mod db;
pub mod handlers;
pub mod error;
pub mod state;
pub mod services;
pub mod dtos;
pub mod migration;
pub use actix_web::App;

async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("index.html")?)
}
async fn style(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("style.css")?)
}
async fn js(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("index.js")?)
}

pub fn start_server(tcp_listener: TcpListener, db: DatabaseConnection) -> Result<Server, std::io::Error> {
    let state = state::AppState::new(db);
    let server = HttpServer::new(move || {
        
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/", web::get().to(index))
            .route("/style.css", web::get().to(style))
            .route("/index.js", web::get().to(js))
            .service(
                web::scope("/api")
                    .route("/categories", web::get().to(handlers::get_categories))
                    .route("/categories", web::post().to(handlers::create_category))
                    .route("/categories/{id}", web::get().to(handlers::get_category))
                    .route("/categories/{id}", web::put().to(handlers::update_category))
                    .route("/categories/{id}", web::delete().to(handlers::delete_category))
                    
                    .route("/customers", web::get().to(handlers::get_customers))
                    .route("/customers", web::post().to(handlers::create_customer))
                    .route("/customers/{id}", web::get().to(handlers::get_customer))
                    .route("/customers/{id}", web::put().to(handlers::update_customer))
                    .route("/customers/{id}", web::delete().to(handlers::delete_customer))
                    .route("/customers/{id}/orders", web::get().to(handlers::get_customer_orders))
                    
                    .route("/suppliers", web::get().to(handlers::get_suppliers))
                    .route("/suppliers", web::post().to(handlers::create_supplier))
                    .route("/suppliers/{id}", web::get().to(handlers::get_supplier))
                    .route("/suppliers/{id}", web::put().to(handlers::update_supplier))
                    .route("/suppliers/{id}", web::delete().to(handlers::delete_supplier))
                    .route("/suppliers/{id}/products", web::get().to(handlers::get_supplier_products))
                    
                    .route("/products", web::get().to(handlers::get_products))
                    .route("/products", web::post().to(handlers::create_product))
                    .route("/products/{id}", web::get().to(handlers::get_product))
                    .route("/products/{id}", web::put().to(handlers::update_product))
                    .route("/products/{id}", web::delete().to(handlers::delete_product))
                    
                    .route("/orders", web::get().to(handlers::get_orders))
                    .route("/orders", web::post().to(handlers::create_order))
                    .route("/orders/{id}", web::get().to(handlers::get_order))
                    .route("/orders/{id}", web::put().to(handlers::update_order))
                    .route("/orders/{id}", web::delete().to(handlers::delete_order))
                    .route("/orders/{id}/details", web::get().to(handlers::get_order_details))
                    
                    .route("/shipments", web::get().to(handlers::get_shipments))
                    .route("/shipments", web::post().to(handlers::create_shipment))
                    .route("/shipments/{id}", web::get().to(handlers::get_shipment))
                    .route("/shipments/{id}", web::put().to(handlers::update_shipment))
                    .route("/shipments/{id}", web::delete().to(handlers::delete_shipment))
                    .route("/shipments/{id}/details", web::get().to(handlers::get_shipment_details))
            )
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
