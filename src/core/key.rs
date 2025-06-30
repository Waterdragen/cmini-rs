pub type Key = char;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct KeyPat(pub Key);

impl PartialEq<Key> for KeyPat {
    fn eq(&self, other: &Key) -> bool {
        self.0 == '_' || self.0 == *other
    }
}
