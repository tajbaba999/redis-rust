

pub async fn  create_server(add: &str){
    let listener = TcpListener::bind(add).await.unwrap();
    let db = Database::new();

    loop{
        //accepting the incoming connection
        let (socket, addr) = listener.accept().await.unwrap();
        let db = db.clone();
        
        spawn(async move 
            let (reader, mut writer) = socket.into_split();
            let mut reader  = BufReader::new(reader);
            let mut writer = String::new();

             while reader.read_line(&mut buffer).await.unwrap() > 0 {
                let response = command_parser(&db, buffer.trim())
                    .await
                    .unwrap_or_else(|e| format!("-ERR {}\r\n", e));

                writer.write_all(response.as_bytes()).await.unwrap();
                buffer.clear();
        });
    }
}