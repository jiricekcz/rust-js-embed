mod vec2d;

use std::rc::Rc;

use deno_core::{
    FsModuleLoader, JsRuntime, OpState, PollEventLoopOptions, RuntimeOptions,
    anyhow::{Context, Error, Ok, Result},
    extension, op2,
    v8::{self},
};
use vec2d::Vec2D;

fn main() -> Result<(), Error> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let future = async {
        let (mut js_runtime, mod_id) = init_runtime().await?;

        let t0 = std::time::Instant::now();

        let global = js_runtime.get_module_namespace(mod_id).unwrap();
        let scope = &mut js_runtime.handle_scope();

        let scope_resolved = std::time::Instant::now();

        let func_key = v8::String::new(scope, "invert").unwrap();
        let func = global.open(scope).get(scope, func_key.into()).unwrap();
        let func = v8::Local::<v8::Function>::try_from(func).unwrap();

        let function_resolved = std::time::Instant::now();

        let n = v8::Integer::new(scope, 1_000_000_000).into();
        let global_this = v8::Local::new(scope, &global).into();

        let func_prepared = std::time::Instant::now();

        let func_res = func.call(scope, global_this, &[n]).unwrap();

        let func_called = std::time::Instant::now();

        let func_res = func_res
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);

        let output_processed = std::time::Instant::now();

        println!("Function returned: {}", func_res);
        println!("Scope resolved in {:?}", scope_resolved.duration_since(t0));
        println!(
            "Function resolved in {:?}",
            function_resolved.duration_since(scope_resolved)
        );
        println!(
            "Function prepared in {:?}",
            func_prepared.duration_since(function_resolved)
        );
        println!(
            "Function called in {:?}",
            func_called.duration_since(func_prepared)
        );
        println!(
            "Output processed in {:?}",
            output_processed.duration_since(func_called)
        );

        Ok(())
    };
    runtime.block_on(future)?;

    Ok(())
}

extension!(tools, ops = [vec, vec_str], objects = [Vec2D]);

#[op2]
#[cppgc]
fn vec() -> Vec2D {
    Vec2D::new_raw(1.0, 2.0)
}

#[op2(fast)]
fn vec_str(#[cppgc] vec: &Vec2D) -> f64 {
    vec.x as f64 + vec.y as f64
}

async fn init_runtime() -> Result<(JsRuntime, usize)> {
    let t0 = std::time::Instant::now();

    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![tools::init_ops()],
        ..Default::default()
    });

    let js_runtime_built = std::time::Instant::now();

    let main_module = deno_core::resolve_path(
        "./plugins/invert.js",
        &std::env::current_dir().context("Unable to get CWD")?,
    )?;

    let path_resolved = std::time::Instant::now();

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;

    let module_loaded = std::time::Instant::now();

    js_runtime.mod_evaluate(mod_id).await?;

    let module_evaluated = std::time::Instant::now();

    js_runtime
        .run_event_loop(PollEventLoopOptions {
            ..Default::default()
        })
        .await?;

    let event_loop_run = std::time::Instant::now();

    println!("Runtime built in {:?}", js_runtime_built.duration_since(t0));
    println!(
        "Path resolved in {:?}",
        path_resolved.duration_since(js_runtime_built)
    );
    println!(
        "Module loaded in {:?}",
        module_loaded.duration_since(path_resolved)
    );
    println!(
        "Module evaluated in {:?}",
        module_evaluated.duration_since(module_loaded)
    );
    println!(
        "Event loop run in {:?}",
        event_loop_run.duration_since(module_evaluated)
    );
    Ok((js_runtime, mod_id))
}
