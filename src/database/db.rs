use std::{collections::HashMap, sync::{Arc, RwLock}, time::{Duration, Instant}};

pub  struct Database {
    db : Arc<RwLock<HashMap<String, String>>>,
    expiry : Arc<RwLock<HashMap<String, Instant>>>,
}

impl Database {
    pub fn new() -> Self {
        Database { 
            db: Arc::new(RwLock::new(HashMap::new())), 
            expiry: Arc::new(RwLock::new(HashMap::new())),
            list : Arc::new(RwLock::new(HashMap::new())),
            set : Arc::new(RwLock::new(HashMap::new())),
            sorted_set : Arc::new(RwLock::new(HashMap::new())), 
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

    //List
    pub async fn lpush(&self, key : String, value : String){
        let mut list_map = self.list.write().unwrap();
        let list = list_map.entry(key).or_insert(RList::new());
        list.lpush(value);
    }

    pub async fn lpop(&self, key : String){
        let mut list_map = self.list.write().unwrap();
        let list = list_map.get_mut(&key);
        if let Some(list) = list{
            list.lpop();
        }else{
            return None;
        }   
    }

    pub async fn lrange(&self, start : usize, end : usize, key: &str) -> Option<Vec<String>> {
        let mut list_map = self.list.write().unwrap();
        if let Some(list) = list_map.get_mut(key){
            return Some(list.lrange(start, end));
        }else{
            return None;
        }   
    }

    //SET
    pub async fn sadd(&self, key : String, value : String) -> bool {
        let mut set_map = self.set.write().unwrap();
        let set = set_map.entry(key).or_insert(RSets::new());
        set.sadd(value);
    }

    pub async fn srem(&self, key : String, value : String) -> bool {
        let mut set_map = self.set.write().unwrap();
        let set = set_map.get_mut(&key);
        if let Some(set) = set{
            return set.srem(value);
        }else{
            return false;
        }
    }

    pub async fn smembers(&self, key : String) -> Option<Vec<String>> {
        let mut set_map = self.set.write().unwrap();
        let set = set_map.get_mut(&key);
        if let Some(set) = set{
            return Some(set.smembers());
        }else{
            return None;
        }
    }   

    pub async fn sismember(&self, key : String, value : String) -> bool{
        let mut set_map = self.set.write().unwrap();
        if let Some(set) = set_map.get_mut(&key){
            return set.sismember(&value);
        }else{
            return false;
        }
    }

    //Sorted Set
    pub async fn zadd(&self, key : String, score: f64, member : String) -> bool{
        let mut sorted_set_map = self.sorted_set.write().unwrap();
        let sorted_set = sorted_set_map.entry(key).or_insert(RSortedSet::new());
        sorted_set.zadd(score, member);
    }

    pub async fn zrem(&self, key : String, member : String) -> bool{
        let mut sorted_set_map = self.sorted_set.write().await;
        if let Some(sorted_set) = sorted_set_map.get_mut(&key){
            return sorted_set.zrem(member);
        }else{
            return false;
        }   
    }

    pub async fn zrange(&self, key : String, start : usize, end : usize) -> Option<Vec<String>> {
        let ss_map = self.sorted_set.read().await;
        if let Some(sorted_set) = ss_map.get(key) {
            Some(sorted_set.zrange(start, end))
        } else {
            None
        }
    }

    pub async fn zscore(&self, key : String, member : String) -> Option<f64> {
       let ss_map = self.sorted_set.read().await;
        if let Some(sorted_set) = ss_map.get(key) {
            sorted_set.zscore(member)
        } else {
            None
        }
    }
}