use std::ops::Deref;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::prelude::RwLock;
use crate::util::jsons::read_json;
use crate::util::memory::AUTHORS;

#[derive(Debug, Error)]
pub enum PermissionError {
    #[error("Unauthorized")]
    NotAdmin,
    #[error("Unauthorized")]
    NotOwner,
    #[error("Error: admin not found")]
    RemoveNonAdmin,
    #[error("Error: already an admin")]
    AddAlreadyAdmin,
    #[error("Error: invalid ID")]
    AddNonAklAuthor,
    #[error("Error: owner cannot remove self")]
    OwnerRemoveSelf,
}

#[derive(Copy, Clone)]
enum Permission {
    User,
    Admin,
    Owner,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Admins(Arc<RwLock<BaseAdmins>>);

#[derive(Serialize, Deserialize)]
pub struct BaseAdmins {
    owner: u64,
    admins: Vec<u64>,
}

impl Admins {
    #[track_caller]
    pub fn open(path: &str) -> Self {
        let admins = read_json::<Self>(path);
        {
            let read = admins.0.read();
            assert!(!read.admins.contains(&read.owner));
        }
        admins
    }
    fn permission_of(&self, id: u64) -> Permission {
        let admins = self.0.read();
        if admins.owner == id {
            return Permission::Owner;
        }
        if admins.admins.contains(&id) {
            return Permission::Admin;
        }
        Permission::User
    }
    fn raw_try_add(&self, id: u64) -> Result<(), PermissionError> {
        let mut admins = self.0.write();
        if admins.admins.contains(&id) {
            return Err(PermissionError::AddAlreadyAdmin);
        }
        admins.admins.push(id);
        Ok(())
    }
    fn raw_try_remove(&self, id: u64) -> Result<(), PermissionError> {
        let authors = AUTHORS.read();
        if authors.get_name(id).is_none() {
            return Err(PermissionError::AddNonAklAuthor);
        }

        let mut admins = self.0.write();
        match admins.admins.iter().position(|admin| *admin == id) {
            None => Err(PermissionError::RemoveNonAdmin),
            Some(index) => {
                admins.admins.remove(index);
                Ok(())
            }
        }
    }
    pub fn contains(&self, id: u64) -> bool {
        matches!(self.permission_of(id), Permission::Admin | Permission::Owner)
    }
    pub fn count(&self) -> usize {
        let admins = self.0.read();
        admins.admins.len() + 1
    }
    pub fn add(&self, caller: u64, target: u64) -> Result<(), PermissionError> {
        match self.permission_of(caller) {
            Permission::User | Permission::Admin => {
                Err(PermissionError::NotOwner)
            }
            Permission::Owner => self.raw_try_add(target),
        }
    }
    pub fn remove(&self, caller: u64, target: u64) -> Result<(), PermissionError> {
        match self.permission_of(caller) {
            Permission::User => Err(PermissionError::NotAdmin),
            Permission::Admin => self.raw_try_remove(target),
            Permission::Owner => {
                match self.permission_of(target) {
                    Permission::Owner => Err(PermissionError::OwnerRemoveSelf),
                    _ => self.raw_try_remove(target),
                }
            }
        }
    }
    pub fn list(&self, caller: u64) -> Result<Vec<String>, PermissionError> {
        if matches!(self.permission_of(caller), Permission::User) {
            return Err(PermissionError::NotAdmin);
        }
        let authors = AUTHORS.read();
        let admins = self.0.read();
        let mut admin_names = admins.admins.iter()
            .map(|id| authors.get_name(*id).unwrap().to_owned())
            .collect::<Vec<String>>();
        admin_names.push(authors.get_name(admins.owner).unwrap().to_owned());
        Ok(admin_names)
    }
}

impl Deref for Admins {
    type Target = Arc<RwLock<BaseAdmins>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}