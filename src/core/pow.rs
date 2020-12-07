/// 共识机制：工作量证明
///
/// 哈希函数：
///     1. 输入x可以是任意长度的字符串
///     2. 输出结果即H(x)的长度是固定的
///     3. 计算H(x)的过程是高效的（对于长度为n的字符串x，计算出H(x)的时间复杂度应为O(n)）
///
/// 比特币的哈希函数需要另外具备以下的性质：
///     1. 免碰撞，即不会出现输入 x ≠ y ，但是 H(x) = H(y)
/// 该特点在理论上并不成立，比如SHA256算法，会有 2^256 种输出，如果进行2^256 +1次输入，那么必然会产生一次碰撞；
/// 但 2^256 已经是天文数字，发生碰撞的几率是极其小的。
///     2. 隐匿性，即对于一个给定的输出结果 H(x) ，想要逆推出输入 x ，在计算上是不可能的。
///     3. 不存在比穷举更好的方法，可以使哈希结果H(x)落在特定的范围。
///
/// 找到前导为16个0的哈希散列，预期大概要进行 2^16 次尝试（哈希值的伪随机特性使得我们可以做概率估算）
///
/// What network hash rate results in a given difficulty?
/// https://en.bitcoin.it/wiki/Difficulty
/// 期望 10 minutes 一个block ，所以 2016 blocks 正好要两周，If the previous 2016 blocks took more than
/// two weeks to find, the difficulty is reduced. If they took less than two weeks, the difficulty is increased.
///
///
///
use crate::core::block::Block;
use crate::utils::coder;
use crate::utils::key::U256;

const MAX_NONCE: u32 = 0x7FFFFFFF;

pub struct ProofOfWork {
    /// target is a 256 bit number
    /// - difficulty = difficulty_1_target / current_target
    /// 这里target直接取 difficulty_1_target
    ///
    /// bdiff : difficulty_1_target : 0x1d00ffff
    /// 0x00ffff * 2**(8*(0x1d - 3)) = 0x00000000FFFF0000000000000000000000000000000000000000000000000000
    /// pdiff : difficulty_1_target
    /// 0x00000000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    target: U256,
}

impl ProofOfWork {
    /// # Arguments
    /// * bits - BlockChain.curr_bits
    /// https://en.bitcoin.it/wiki/Difficulty
    ///
    /// The compact format of target is 特殊的 floating-point encoding using 24 bits mantissa,
    /// the first 8 bits are exponent (where only the 5 lowest bits are used) and its base is 256.
    /// - 0x1b0404cb :
    /// 0x0404cb * 2**(8*(0x1b - 3)) = 0x00000000000404CB000000000000000000000000000000000000000000000000
    ///
    pub fn new(bits: u32) -> ProofOfWork {
        let mantissa = bits & 0xFFFFFF;
        // mantissa contains a sign bit in the 24th bit
        // so the largest value for mantissa is 0x7fffff , and the smallest value is 0x800000 （wiki上写错了）
        if mantissa > 0x7FFFFF {
            return ProofOfWork {
                target: Default::default(),
            };
        }

        let exponent = (bits >> 24) as usize;
        if exponent < 3 {
            ProofOfWork {
                target: U256::from(mantissa as u64) >> (8 * (3 - exponent)),
            }
        } else {
            ProofOfWork {
                target: U256::from(mantissa as u64) << (8 * (exponent - 3)),
            }
        }
    }

    pub fn block_header_se(b: &mut Block, nonce: u32) -> Vec<u8> {
        b.header.nonce = nonce;
        coder::serialize(&b.header)
    }

    /// expensive task
    pub fn run(&self, b: &mut Block) {
        let mut nonce = 0u32;
        while nonce <= MAX_NONCE {
            let data = Self::block_header_se(b, nonce);
            // 应该要双重SHA256运算（即SHA256(SHA256(Block_Header))）
            let mut hash = coder::get_hash(&data);

            let hash_uint = U256::from(hash);
            // 计算成功
            if hash_uint <= self.target {
                println!("pow success, hash:  {:?}", hash);
                b.hash = hash;

                break;
            } else {
                nonce += 1;
            }
        }
    }
}
