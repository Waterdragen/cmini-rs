#[cfg(test)]
pub mod test {
    use std::hash::{Hash, Hasher};
    use fxhash::FxHasher;


    #[repr(u8)]
    enum Number {
        One,
        Two,
        Three,
    }

    impl Number {
        #[inline]
        fn discriminant(&self) -> u8 {
            unsafe { *<*const _>::from(self).cast::<u8>() }
        }
    }

    impl Hash for Number {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write_u8(self.discriminant());
        }
    }

    #[test]
    fn test_hasher() {
        println!("{:?}", Number::Three.discriminant());
    }
}