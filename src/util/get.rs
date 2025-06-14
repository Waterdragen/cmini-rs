use crate::util::core::FxIndexMap;
use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

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
        self.checked().unwrap_or_else(|| panic!("Cannot get {:?}", &*self.1))
    }
}

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
            None => panic!("Cannot get {:?}", &*self.1),
            Some(item) => item
        }
    }
}

