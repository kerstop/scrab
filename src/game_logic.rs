use std::{sync::{Arc, RwLock}, time::Duration};

use log::info;

use scrab_types::World;
use wasmer::{Store, Cranelift, Module, imports, Function, Instance, wat2wasm};



pub fn main_loop(world: Arc<RwLock<World>>) {
    
    loop {
        info!("Starting tick {}", world.read().unwrap().current_tick);
        execute_wasm();
        std::thread::sleep(Duration::from_secs(2));
        
        info!("Finishing tick {}", world.read().unwrap().current_tick);
        world.write().unwrap().current_tick += 1;
    }
}

fn execute_wasm() {
let wasm_bytes = wat2wasm(
        br#"
(module
  ;; First we define a type with no parameters and no results.
  (type $no_args_no_rets_t (func (param) (result)))

  ;; Then we declare that we want to import a function named "env" "say_hello" with
  ;; that type signature.
  (import "env" "say_hello" (func $say_hello (type $no_args_no_rets_t)))

  ;; Finally we create an entrypoint that calls our imported function.
  (func $run (type $no_args_no_rets_t)
    (call $say_hello))
  ;; And mark it as an exported function named "run".
  (export "run" (func $run)))
"#,
    ).unwrap();

    let mut store = Store::new(Cranelift::default());

    let module = Module::new(&store, wasm_bytes).expect("Wasm should be valid");

    fn hello_world() {
        println!("hello_world")
    }

    let import_objects = imports! {
        "env" => {
            "say_hello" => Function::new_typed(&mut store, hello_world),
        }
    };

    let instance = Instance::new(&mut store, &module, &import_objects).unwrap();

    instance.exports.get_typed_function::<(),()>(&mut store, "run").unwrap().call(&mut store).unwrap();
}