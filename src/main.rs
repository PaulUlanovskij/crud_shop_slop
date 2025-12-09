use std::net::TcpListener;

use crud_shop_slop::{
    config::read_config,
    db::*,
    start_server,
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config = read_config().expect("Failed to read server config");
    let server_address = format!("{}:{}", config.application.host, config.application.port);

    let listener = TcpListener::bind(server_address).expect("Failed to bind server port");
    let connection_string = connection_string(
        config.db.user,
        config.db.password,
        config.db.host,
        config.db.port,
        Some(config.db.name),
    );
    let pool = connect_database(connection_string)
        .await
        .expect("Failed to connect to database");

        migrate(&pool).await.expect("Failed to run migrations on database");
        start_server(listener, pool.into())
        .expect("Failed to start server")
        .await
}
