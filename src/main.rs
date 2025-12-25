use std::sync::Arc;

mod command;
mod database;
mod http_api;
mod server;

#[tokio::main]
async fn main() {
    let db = Arc::new(database::Database::new());
    
    // Clone for TCP server
    let db_tcp = db.clone();
    
    // Start TCP Redis server in background
    let tcp_handle = tokio::spawn(async move {
        let addr = "127.0.0.1:6379";
        eprintln!("Starting Redis TCP server at {}", addr);
        server::create_server(addr, db_tcp).await;
    });
    
    // Start HTTP API server
    let http_addr = "127.0.0.1:3000";
    eprintln!("Starting HTTP API server at {}", http_addr);
    http_api::create_http_server(http_addr, db).await;
    
    tcp_handle.await.unwrap();
}
