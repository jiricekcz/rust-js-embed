use std::rc::Rc;

use deno_core::{
    FsModuleLoader, JsRuntime, PollEventLoopOptions, RuntimeOptions,
    anyhow::{Context, Error},
    v8::{self, Handle},
};

fn main() -> Result<(), Error> {
    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    });

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let main_module = deno_core::resolve_path(
        "./module.js",
        &std::env::current_dir().context("Unable to get CWD")?,
    )?;

    let future = async move {
        let mod_id = js_runtime.load_main_es_module(&main_module).await?;
        let result = js_runtime.mod_evaluate(mod_id).await?;
        js_runtime
            .run_event_loop(PollEventLoopOptions {
                ..Default::default()
            })
            .await?;

        let global = js_runtime.get_module_namespace(mod_id).unwrap();
        let scope = &mut js_runtime.handle_scope();

        let func_key = v8::String::new(scope, "sum").unwrap();
        let func = global.open(scope).get(scope, func_key.into()).unwrap();
        let func = v8::Local::<v8::Function>::try_from(func).unwrap();

        let a = v8::Integer::new(scope, 5).into();
        let b = v8::Integer::new(scope, 2).into();
        let func_res = func.call(scope, global.open(scope)., &[a, b]).unwrap();
        let func_res = func_res
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);
        println!("Function returned: {}", func_res);

        result.await?
    };
    runtime.block_on(future)
}
