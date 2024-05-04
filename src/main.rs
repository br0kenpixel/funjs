use deno_core::{error::AnyError, PollEventLoopOptions};
use std::{
    env::{args, current_dir},
    process::exit,
    rc::Rc,
};

mod fs_impl;
mod math_impl;
mod rand_impl;
mod runtime_impl;

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path, &current_dir().unwrap())?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![
            fs_impl::fs_ext::init_ops_and_esm(),
            math_impl::math_ext::init_ops_and_esm(),
            rand_impl::rand_ext::init_ops_and_esm(),
            runtime_impl::runtime_ext::init_ops_and_esm(),
        ],
        ..Default::default()
    });

    js_runtime
        .execute_script("[funjs:runtime.js]", include_str!("./runtime.js"))
        .unwrap();

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime
        .run_event_loop(PollEventLoopOptions::default())
        .await?;

    result.await
}

fn main() {
    let Some(source) = args().nth(1) else {
        eprintln!("Usage: {} [FILE]", args().next().unwrap());
        exit(1);
    };

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(error) = runtime.block_on(run_js(&source)) {
        eprintln!("Error: {}", error);
    }
}
