use deno_core::op2;

#[op2]
#[serde]
fn op_math_sum(#[serde] nums: Vec<i64>) -> i64 {
    nums.iter().sum()
}

#[op2]
#[serde]
fn op_math_min(#[serde] nums: Vec<i64>) -> i64 {
    nums.into_iter().min().unwrap_or(0)
}

#[op2]
#[serde]
fn op_math_max(#[serde] nums: Vec<i64>) -> i64 {
    nums.into_iter().max().unwrap_or(0)
}

deno_core::extension!(math_ext, ops = [op_math_max, op_math_min, op_math_sum]);
