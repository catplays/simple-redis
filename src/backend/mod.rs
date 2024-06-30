use std::{ops::Deref, sync::Arc};

use dashmap::DashMap;

use crate::RespFrame;

#[derive(Debug, Clone)]
pub struct Backend(Arc<BackendInner>);

#[derive(Debug)]
pub struct BackendInner {
    pub map: DashMap<String, RespFrame>,
    pub hmap: DashMap<String, DashMap<String, RespFrame>>,
}

// 解引用，使得Backend当做BackendInner使用
impl Deref for Backend {
    type Target = BackendInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Default for Backend {
    fn default() -> Self {
        Self(Arc::new(BackendInner::default()))
    }
}

impl Default for BackendInner {
    fn default() -> Self {
        Self {
            map: DashMap::new(),
            hmap: DashMap::new(),
        }
    }
}

impl Backend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<RespFrame> {
        self.map.get(key).map(|v| v.value().clone())
    }

    pub fn set(&self, key: String, value: RespFrame) {
        self.map.insert(key, value);
    }

    pub fn hget(&self, key:&str, field:&str) -> Option<RespFrame> {
        self.hmap
        .get(key)
        .and_then(|v| v.get(field).map(|v| v.value().clone()))
    }
    pub fn hset(&self, key:String, field:String, value:RespFrame){
       let hmap = self.hmap.entry(key).or_default();
        hmap.insert(field, value);
    }
}
