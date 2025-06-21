pub use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};
use std::sync::RwLock as StdRwLock;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct RwLock<T>(StdRwLock<T>);

impl<T> RwLock<T> {
    pub fn new(t: T) -> Self {
        Self(StdRwLock::new(t))
    }
    pub fn read(&self) -> RwLockReadGuard<T> {
        self.0.read().unwrap()
    }
    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.0.write().unwrap()
    }
}
