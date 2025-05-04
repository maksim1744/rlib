pub mod mersenne_twister;
mod mrand;
pub mod randomable;

pub type Rng = mersenne_twister::MersenneTwister<
    64,
    312,
    156,
    31,
    0xB5026F5AA96619E9,
    0x71D67FFFEDA60000,
    0xFFF7EEE000000000,
    17,
    37,
    29,
    0x5555555555555555,
    43,
    6364136223846793005,
>;
pub use mrand::Rand;
