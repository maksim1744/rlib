pub mod lcg;
pub mod randomable;

pub type Rng = lcg::LinearCongruentialGenerator64<6364136223846793005, 1442695040888963407>;
