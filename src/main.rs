use std::rc::Rc;

use deno_core::{
    FsModuleLoader, JsRuntime, PollEventLoopOptions, RuntimeOptions,
    anyhow::{Context, Error, Result},
    v8::{self, Handle},
};

fn main() -> Result<(), Error> {
    let future = async move {
        let global = js_runtime.get_module_namespace(mod_id).unwrap();
        let scope = &mut js_runtime.handle_scope();

        let func_key = v8::String::new(scope, "invert").unwrap();
        let func = global.open(scope).get(scope, func_key.into()).unwrap();
        let func = v8::Local::<v8::Function>::try_from(func).unwrap();

        let n = v8::Integer::new(scope, 5).into();
        let global_this = v8::Local::new(scope, global).into();
        let func_res = func.call(scope, global_this, &[n]).unwrap();
        let func_res = func_res
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);
        println!("Function returned: {}", func_res);

        result.await
    };
    runtime.block_on(future).map_err(|err| err.into())
}

async fn init_runtime() -> Result<JsRuntime> {
    let t0 = std::time::Instant::now();

    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    });

    let js_runtime_built = std::time::Instant::now();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let tokio_runtime_built = std::time::Instant::now();

    let main_module = deno_core::resolve_path(
        "./plugins/invert.js",
        &std::env::current_dir().context("Unable to get CWD")?,
    )?;

    let path_resolved = std::time::Instant::now();

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;

    let module_loaded = std::time::Instant::now();

    let evaluate_result = js_runtime.mod_evaluate(mod_id);

    js_runtime
        .run_event_loop(PollEventLoopOptions {
            ..Default::default()
        })
        .await?;

    Ok(js_runtime)
}
