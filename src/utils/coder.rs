use bincode;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::{Deserialize, Serialize};

pub fn serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    bincode::serialize(value).unwrap()
}

pub fn deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    bincode::deserialize(bytes).unwrap()
}

/// 8 x 32 = 256位
pub fn get_hash(value: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    let mut hash = [0u8; 32];
    hasher.result(&mut hash);

    hash
}

#[cfg(test)]
mod tests {
    use super::{deserialize, serialize};
    // 在同一模块（要序列化）结构体中声明 derive macro
    use serde::{Deserialize, Serialize};

    // PartialEq 允许结构体部分属性没有实现 PartialEq ，比如用于 ！= 运算
    // Eq 要求结构体所有属性必须都实现 Eq , Eq 没有实现自己与自己 == ？
    // 类似的 PartialOrd, Ord, 要研究
    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn coder_works() {
        let point = Point { x: 1, y: 1 };
        let se = serialize(&point);
        let de: Point = deserialize(&se);

        assert_eq!(de, point);
    }
}
