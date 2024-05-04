use deno_core::op2;
use rand::{
    distributions::{Alphanumeric, DistString},
    prelude::*,
};

#[op2]
#[serde]
fn op_rand_range(#[serde] min: i64, #[serde] max: i64, inclusive: bool) -> i64 {
    let mut rng = rand::thread_rng();

    if inclusive {
        rng.gen_range(min..=max)
    } else {
        rng.gen_range(min..max)
    }
}

#[op2]
#[string]
fn op_rand_string(len: u8) -> String {
    let mut rng = rand::thread_rng();

    Alphanumeric.sample_string(&mut rng, len as _)
}

deno_core::extension!(rand_ext, ops = [op_rand_range, op_rand_string]);
