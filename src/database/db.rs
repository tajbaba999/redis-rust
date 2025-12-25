use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::database::data_structure::{RList, RSets, RSortedSet};

#[derive(Clone)]
pub struct Database {
    db: Arc<RwLock<HashMap<String, String>>>,
    expiry: Arc<RwLock<HashMap<String, Instant>>>,
    list: Arc<RwLock<HashMap<String, RList>>>,
    set: Arc<RwLock<HashMap<String, RSets>>>,
    sorted_set: Arc<RwLock<HashMap<String, RSortedSet>>>,
}

impl Database {
    pub fn new() -> Self {
        Database { 
            db: Arc::new(RwLock::new(HashMap::new())), 
            expiry: Arc::new(RwLock::new(HashMap::new())),
            list: Arc::new(RwLock::new(HashMap::new())),
            set: Arc::new(RwLock::new(HashMap::new())),
            sorted_set: Arc::new(RwLock::new(HashMap::new())), 
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let now = Instant::now();

        // Check expiry - must drop guard before await
        let is_expired = {
            let exp_map = self.expiry.read().unwrap();
            if let Some(exp) = exp_map.get(key) {
                now > *exp
            } else {
                false
            }
        };
        
        if is_expired {
            self.delete(key).await;
            return None;
        }

        let db = self.db.read().unwrap();
        db.get(key).cloned()
    }
    
    pub async fn set(&self, key: String, value: String, ttl: Option<u64>) {
        let mut db_map = self.db.write().unwrap();
        db_map.insert(key.clone(), value);

        if let Some(sec) = ttl {
            let exp_time = Instant::now() + Duration::from_secs(sec);
            let mut exp_map = self.expiry.write().unwrap();
            exp_map.insert(key, exp_time);
        }
    }


    pub async fn is_expired(&self, key: &str) -> bool {
        let exp_map = self.expiry.read().unwrap();
        if let Some(exp_time) = exp_map.get(key) {
            return Instant::now() > *exp_time;
        }
        false
    }

    pub async fn delete(&self, key: &str) -> bool {
        let mut db = self.db.write().unwrap();
        let mut expiry = self.expiry.write().unwrap();

        let existed = db.remove(key).is_some();
        expiry.remove(key);
        existed
    }

    // List operations
    pub async fn lpush(&self, key: String, value: String) -> usize {
        let mut list_map = self.list.write().unwrap();
        let list = list_map.entry(key).or_insert_with(RList::new);
        list.lpush(value);
        list.list.len()
    }

    pub async fn rpush(&self, key: String, value: String) -> usize {
        let mut list_map = self.list.write().unwrap();
        let list = list_map.entry(key).or_insert_with(RList::new);
        list.rpush(value);
        list.list.len()
    }

    pub async fn lpop(&self, key: &str) -> Option<String> {
        let mut list_map = self.list.write().unwrap();
        if let Some(list) = list_map.get_mut(key) {
            list.lpop()
        } else {
            None
        }
    }

    pub async fn rpop(&self, key: &str) -> Option<String> {
        let mut list_map = self.list.write().unwrap();
        if let Some(list) = list_map.get_mut(key) {
            list.rpop()
        } else {
            None
        }
    }

    pub async fn lrange(&self, key: &str, start: i64, end: i64) -> Option<Vec<String>> {
        let list_map = self.list.read().unwrap();
        if let Some(list) = list_map.get(key) {
            Some(list.lrange(start, end))
        } else {
            None
        }
    }

    // SET operations
    pub async fn sadd(&self, key: String, value: String) -> bool {
        let mut set_map = self.set.write().unwrap();
        let set = set_map.entry(key).or_insert_with(RSets::new);
        set.sadd(value)
    }

    pub async fn srem(&self, key: &str, value: String) -> bool {
        let mut set_map = self.set.write().unwrap();
        if let Some(set) = set_map.get_mut(key) {
            set.srem(value)
        } else {
            false
        }
    }

    pub async fn smembers(&self, key: &str) -> Option<Vec<String>> {
        let set_map = self.set.read().unwrap();
        if let Some(set) = set_map.get(key) {
            Some(set.smembers())
        } else {
            None
        }
    }

    pub async fn sismember(&self, key: &str, value: &str) -> bool {
        let set_map = self.set.read().unwrap();
        if let Some(set) = set_map.get(key) {
            set.sismember(value)
        } else {
            false
        }
    }

    // Sorted Set operations
    pub async fn zadd(&self, key: String, score: f64, member: String) -> bool {
        let mut sorted_set_map = self.sorted_set.write().unwrap();
        let sorted_set = sorted_set_map.entry(key).or_insert_with(RSortedSet::new);
        sorted_set.zadd(score, member)
    }

    pub async fn zrem(&self, key: &str, member: String) -> bool {
        let mut sorted_set_map = self.sorted_set.write().unwrap();
        if let Some(sorted_set) = sorted_set_map.get_mut(key) {
            sorted_set.zrem(member)
        } else {
            false
        }
    }

    pub async fn zrange(&self, key: &str, start: usize, end: usize) -> Option<Vec<String>> {
        let ss_map = self.sorted_set.read().unwrap();
        if let Some(sorted_set) = ss_map.get(key) {
            Some(sorted_set.zrange(start, end))
        } else {
            None
        }
    }

    pub async fn zscore(&self, key: &str, member: &str) -> Option<f64> {
        let ss_map = self.sorted_set.read().unwrap();
        if let Some(sorted_set) = ss_map.get(key) {
            sorted_set.zscore(member)
        } else {
            None
        }
    }
}