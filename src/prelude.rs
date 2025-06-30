pub use std::sync::{Arc, RwLock as StdRwLock, Mutex as StdMutex};
use serde::{Deserialize, Serialize};
use std::sync::{RwLockReadGuard, RwLockWriteGuard, MutexGuard};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
/// Wrapper around [`RwLock`](StdRwLock) that doesn't require `.unwrap()`
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

#[repr(transparent)]
/// Wrapper around [`Mutex`](StdMutex) that doesn't require `.unwrap()`
pub struct Mutex<T>(StdMutex<T>);

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self(StdMutex::new(t))
    }
    pub fn lock(&self) -> MutexGuard<T> {
        self.0.lock().unwrap()
    }
}

pub use crate::message::{Message, BoundedResponse, BoundedResponseVec};
pub use crate::util::Commandable;
pub use once_cell::sync::Lazy;
