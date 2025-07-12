use crate::FxIndexMap;
use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

/// This is a workaround of this problem:
/// - we can't return a reference from a guard
/// - `RwLockReadGuard::map` is unstable
/// # Workaround
/// we can return the guard itself (wrapped), and do something while dereferencing
pub struct Get<'a, V: 'a>(pub RwLockReadGuard<'a, FxIndexMap<String, V>>, pub Cow<'a, str>);

impl<'a, V: 'a> Get<'a, V> {
    pub fn checked(&self) -> Option<&V> {
        self.0.get(&*self.1)
    }
}

impl<'a, V: 'a> Deref for Get<'a, V> {
    type Target = V;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        // unwrap on deref - more ergonomic for find() which is infallible on non empty string
        self.checked().unwrap_or_else(|| panic!("Cannot get {:?}", &*self.1))
    }
}

/// This is a workaround of this problem:
/// - we can't return a reference from a guard
/// - `RwLockWriteGuard::map` is unstable
/// # Workaround
/// we can return the guard itself (wrapped), and do something while dereferencing
pub struct GetMut<'a, V: 'a>(pub RwLockWriteGuard<'a, FxIndexMap<String, V>>, pub Cow<'a, str>);

impl<'a, V: 'a> GetMut<'a, V> {
    pub fn checked(&mut self) -> Option<&mut V> {
        self.0.get_mut(&*self.1)
    }
}

impl<'a, V: 'a> Deref for GetMut<'a, V> {
    type Target = V;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.0.get(&*self.1).unwrap_or_else(|| panic!("Cannot get {:?}", &*self.1))
    }
}

impl<'a, V: 'a> DerefMut for GetMut<'a, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.0.get_mut(&*self.1) {
            // unwrap on deref - more ergonomic for find_mut() which is infallible on non empty string
            None => panic!("Cannot get {:?}", &*self.1),
            Some(item) => item
        }
    }
}