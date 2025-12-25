use crate::database::Database;

pub async fn command_parser(db: &Database, command: &str) -> Result<String, String> {
    let splitted_command: Vec<&str> = command.split_whitespace().collect();
    
    match splitted_command.as_slice() {
        // String operations
        ["SET", key, value, "EX", ttl] => {
            let ttl = ttl.parse::<u64>().map_err(|_| "Invalid TTL value")?;
            db.set(key.to_string(), value.to_string(), Some(ttl)).await;
            Ok("+OK\r\n".to_string())
        }
        ["SET", key, value] => {
            db.set(key.to_string(), value.to_string(), None).await;
            Ok("+OK\r\n".to_string())
        }
        ["GET", key] => {
            if let Some(value) = db.get(key).await {
                Ok(format!("${}\r\n{}\r\n", value.len(), value))
            } else {
                Ok("$-1\r\n".to_string())
            }
        }
        ["DEL", key] => {
            if db.delete(key).await {
                Ok(":1\r\n".to_string()) 
            } else {
                Ok(":0\r\n".to_string()) 
            }
        }

        // List operations
        ["LPUSH", key, value] => {
            let len = db.lpush(key.to_string(), value.to_string()).await;
            Ok(format!(":{}\r\n", len))
        }
        ["RPUSH", key, value] => {
            let len = db.rpush(key.to_string(), value.to_string()).await;
            Ok(format!(":{}\r\n", len))
        }
        ["LPOP", key] => {
            if let Some(value) = db.lpop(key).await {
                Ok(format!("${}\r\n{}\r\n", value.len(), value))
            } else {
                Ok("$-1\r\n".to_string())
            }
        }
        ["RPOP", key] => {
            if let Some(value) = db.rpop(key).await {
                Ok(format!("${}\r\n{}\r\n", value.len(), value))
            } else {
                Ok("$-1\r\n".to_string())
            }
        }
        ["LRANGE", key, start, end] => {
            let start = start.parse::<i64>().map_err(|_| "Invalid start index")?;
            let end = end.parse::<i64>().map_err(|_| "Invalid end index")?;
            if let Some(values) = db.lrange(key, start, end).await {
                let mut response = format!("*{}\r\n", values.len());
                for v in values {
                    response.push_str(&format!("${}\r\n{}\r\n", v.len(), v));
                }
                Ok(response)
            } else {
                Ok("*0\r\n".to_string())
            }
        }

        // Set operations
        ["SADD", key, value] => {
            let added = db.sadd(key.to_string(), value.to_string()).await;
            Ok(format!(":{}\r\n", if added { 1 } else { 0 }))
        }
        ["SREM", key, value] => {
            let removed = db.srem(key, value.to_string()).await;
            Ok(format!(":{}\r\n", if removed { 1 } else { 0 }))
        }
        ["SISMEMBER", key, value] => {
            let is_member = db.sismember(key, value).await;
            Ok(format!(":{}\r\n", if is_member { 1 } else { 0 }))
        }
        ["SMEMBERS", key] => {
            if let Some(members) = db.smembers(key).await {
                let mut response = format!("*{}\r\n", members.len());
                for m in members {
                    response.push_str(&format!("${}\r\n{}\r\n", m.len(), m));
                }
                Ok(response)
            } else {
                Ok("*0\r\n".to_string())
            }
        }

        // Sorted Set operations
        ["ZADD", key, score, member] => {
            let score = score.parse::<f64>().map_err(|_| "Invalid score")?;
            let added = db.zadd(key.to_string(), score, member.to_string()).await;
            Ok(format!(":{}\r\n", if added { 1 } else { 0 }))
        }
        ["ZREM", key, member] => {
            let removed = db.zrem(key, member.to_string()).await;
            Ok(format!(":{}\r\n", if removed { 1 } else { 0 }))
        }
        ["ZRANGE", key, start, end] => {
            let start = start.parse::<usize>().map_err(|_| "Invalid start index")?;
            let end = end.parse::<usize>().map_err(|_| "Invalid end index")?;
            if let Some(members) = db.zrange(key, start, end).await {
                let mut response = format!("*{}\r\n", members.len());
                for m in members {
                    response.push_str(&format!("${}\r\n{}\r\n", m.len(), m));
                }
                Ok(response)
            } else {
                Ok("*0\r\n".to_string())
            }
        }
        ["ZSCORE", key, member] => {
            if let Some(score) = db.zscore(key, member).await {
                Ok(format!("${}\r\n{}\r\n", score.to_string().len(), score))
            } else {
                Ok("$-1\r\n".to_string())
            }
        }

        // Ping/Pong for testing
        ["PING"] => Ok("+PONG\r\n".to_string()),
        
        _ => Ok("-ERR Unknown command\r\n".to_string()),
    }   
}