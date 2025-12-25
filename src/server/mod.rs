use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::spawn;

use crate::database::Database;
use crate::command::command_parser;

pub async fn create_server(addr: &str, db: Arc<Database>) {
    let listener = TcpListener::bind(addr).await.unwrap();
    
    println!("Redis TCP server listening on {}", addr);

    loop {
        // Accept incoming connections
        let (socket, client_addr) = listener.accept().await.unwrap();
        let db = db.clone();
        
        println!("New connection from: {}", client_addr);
        
        spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut buffer = String::new();

            while reader.read_line(&mut buffer).await.unwrap() > 0 {
                let response = command_parser(&db, buffer.trim())
                    .await
                    .unwrap_or_else(|e| format!("-ERR {}\r\n", e));

                writer.write_all(response.as_bytes()).await.unwrap();
                buffer.clear();
            }
            
            println!("Connection closed: {}", client_addr);
        });
    }
}