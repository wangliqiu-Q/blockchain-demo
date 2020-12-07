use db_key::Key;

// db-key = "0.1.0"
// pub trait Key<'a>: From<&'a [u8]> + AsRef<[u8]> {}

uint::construct_uint! {
    pub struct U256(4);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct MyKey {
    pub val: U256,
}


impl Key for MyKey {
    fn from_u8(key: &[u8]) -> Self {
        use std::mem::transmute;

        assert_eq!(key.len(), 32);
        let mut result = [0u8; 32];
        result.copy_from_slice(key);

        unsafe { transmute::<[u8; 32], Self>(result) }
    }

    fn as_slice<T, F>(&self, f: F) -> T
    where
        F: Fn(&[u8]) -> T,
    {
        use std::mem::transmute;

        let val = unsafe { transmute::<_, &[u8; 32]>(self) };
        f(val)
    }
}
