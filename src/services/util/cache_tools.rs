use std::collections::HashMap;
use std::hash::Hash;
use chrono::Utc;

pub struct CachedValue<E> {
    pub value: E,
    pub duration: i64,
    pub expire_time: i64,
}

/// create implementation for cached values
impl<E> CachedValue<E> {
    pub fn new(value: E , duration: i64) -> CachedValue<E>{
        return CachedValue {
            value,
            duration,
            expire_time: Utc::now().timestamp_millis() + duration,
        };
    }

    /// update the value for this cached value
    pub fn update(&mut self, value: E) {
        self.value = value;
        self.update_expiration();
    }

    pub fn new_expired(value: E , duration: i64) -> CachedValue<E>{
        let mut value = CachedValue::new(value, duration);
        value.force_expire();
        return value;
    }

    /// force expiration of the current value
    pub fn force_expire(&mut self) {
        self.expire_time = 0;
    }

    /// check if the value is expired
    pub fn expired(&self) -> bool {
        return Utc::now().timestamp_millis() > self.expire_time;
    }

    /// update the expiration
    pub fn update_expiration(&mut self) {
        self.expire_time = Utc::now().timestamp_millis() + self.duration;
    }
}

/// Struct representing a cache
pub struct Cache<K: Eq + Hash, V> {
    map: HashMap<K, CachedValue<V>>,
    duration: i64,
}

impl<K: Eq + Hash, V> Cache<K, V> {
    pub fn new(expire_time: i64) -> Cache<K, V> {
        let map: HashMap<K, CachedValue<V>> = HashMap::new();
        return Cache { map, duration: expire_time };
    }

    /// set the value at the given key to the given value
    pub fn set(&mut self, key: K, value: V) {
        // if the value exists just update the state of it
        if let Some(exists) = self.map.get_mut(&key) {
            exists.update(value);
            return;
        }

        self.map.insert(key, CachedValue::new(value, self.duration));
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