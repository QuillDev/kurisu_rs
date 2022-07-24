use std::collections::HashMap;
use std::hash::Hash;
use chrono::Utc;

pub struct CachedValue<E> {
    pub value: E,
    pub expire_time: i64,
}

pub struct Cache<K: Eq + Hash, V> {
    map: HashMap<K, CachedValue<V>>,
    expire_time: i64,
}

impl<K: Eq + Hash, V> Cache<K, V> {
    pub fn new(expire_time: i64) -> Cache<K, V> {
        let map: HashMap<K, CachedValue<V>> = HashMap::new();
        return Cache { map, expire_time };
    }

    /// set the value at the given key to the given value
    pub fn set(&mut self, key: K, value: V) {

        let insert_value = CachedValue {
            value,
            expire_time: Utc::now().timestamp_millis() + self.expire_time,
        };
        self.map.insert(key, insert_value);
    }

    /// get the value with the given key from the cache
    pub fn get(&self, key: &K) -> Option<&CachedValue<V>> {
        return self.map.get(key);
    }

    /// check if the value at the given key is expired
    /// if the value does not exist, the value is expired.
    pub fn is_expired(&self, key: &K) -> bool {
        return match self.map.get(key) {
            None => true,
            Some(value) => {
                // check if the value is expired (current time > expire time)
                Utc::now().timestamp_millis() > value.expire_time
            }
        }
    }
}