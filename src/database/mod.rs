use std::{collections::HashMap, sync::{Arc, RwLock}, time::{Duration, Instant}};

pub  struct Database {
    db : Arc<RwLock<HashMap<String, String>>>,
    expiry : Arc<RwLock<HashMap<String, Instant>>>,
}

impl Database {
    pub fn new() -> Self {
        Database { 
            db: Arc::new(RwLock::new(HashMap::new())), 
            expiry: Arc::new(RwLock::new(HashMap::new()))
        }
    }

   pub async fn get(&self, key: &str) -> Option<String> {
    let now = Instant::now();

    let exp_map = self.expiry.read().unwrap();
    if let Some(exp) = exp_map.get(key) {
        if now > *exp {
            drop(exp_map);
            self.delete(key).await;
            return None;
        }
    }
    drop(exp_map);

    let db = self.db.read().unwrap();
    db.get(key).cloned()
    }
    
    pub async fn set(&self, key : String, value : String, ttl : Option<u64> ) {
        let mut  db_map = self.db.write().await;
        db_map.insert(key.clone(), value);

        if let Some(sec) = ttl{
            let exp_time = Instant::now() + Duration::from_secs(sec);
            let mut exp_map = self.expiry.write().await;
            exp_map.insert(key, exp_time)
        }
    }


    pub async fn is_expired(&self, key : &str) -> bool {
        let exp_map = self.expiry.read().unwrap();
        if let Some(exp_time) = exp_map.get(key){
            return Instant::now() > exp_time;
        } 
        return false;
    }
    pub async fn delete(&self, key: &str) -> bool{
       let mut db = self.db.write().unwrap();
       let mut expiry = self.db.write().unwrap();

       let exisited = db.remove(key).is_some();
       expiry.remove(key);
       exisited
    }

}