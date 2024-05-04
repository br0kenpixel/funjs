use deno_core::op2;

#[op2]
#[string]
fn op_runtime_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

deno_core::extension!(runtime_ext, ops = [op_runtime_version]);
