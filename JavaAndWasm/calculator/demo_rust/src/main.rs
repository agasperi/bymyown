use calc::{sumar, restar, multiplicar, dividir};
use wasmer::{Store, Module, Instance, Value, imports};

fn main() {
    // Values to use
    let a = 5;
    let b = 3;

    // Using the library in Rust
    println!("Suma: {}", sumar(a, b));
    println!("Resta: {}", restar(a, b));
    println!("Multiplicación: {}", multiplicar(a, b));

    match dividir(a, b) {
        Some(resultado) => println!("División: {}", resultado),
        None => println!("No se puede dividir por cero."),
    }

    let wasm_bytes = std::fs::read("calc.wasm").expect("Failed to read Wasm file");
    let mut store: Store = Store::default();
    let module = Module::new(&store, &wasm_bytes).expect("Failed to compile Wasm module");
    let import_objects = imports! {};
    let instance = Instance::new(&mut store, &module, &import_objects).expect("Failed to instantiate Wasm module");

    println!("Calling Webassembly functions with Wasmer!");
    let sumar_func: &wasmer::Function = instance.exports.get_function("sumar").expect("Failed to find 'sumar' function");
    let restar_func: &wasmer::Function = instance.exports.get_function("restar").expect("Failed to find 'restar' function");
    let multiplicar_func: &wasmer::Function = instance.exports.get_function("multiplicar").expect("Failed to find 'multiplicar' function");

    let results = sumar_func.call(&mut store, &[Value::I32(a),Value::I32(b)]).expect("Failed to call 'sumar' function");
    println!("Suma: {}", results[0].unwrap_i32());
    let results = restar_func.call(&mut store, &[Value::I32(a),Value::I32(b)]).expect("Failed to call 'restar' function");
    println!("Resta: {}", results[0].unwrap_i32());
    let results = multiplicar_func.call(&mut store, &[Value::I32(a),Value::I32(b)]).expect("Failed to call 'multiplicar' function");
    println!("Multiplicación: {}", results[0].unwrap_i32());
}