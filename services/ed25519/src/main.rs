#[cfg(target_arch = "wasm32")]
mod ed25519;

/*
   _initialize function that calls __wasm_call_ctors is required to mitigade memory leak
   that is described in https://github.com/WebAssembly/wasi-libc/issues/298

   In short, without this code rust wraps every export function
   with __wasm_call_ctors/__wasm_call_dtors calls. This causes memory leaks. When compiler sees
   an explicit call to __wasm_call_ctors in _initialize function, it disables export wrapping.

   TODO: remove when updating to marine-rs-sdk with fix
*/
#[cfg(target_arch = "wasm32")]
extern "C" {
    pub fn __wasm_call_ctors();
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
fn _initialize() {
    unsafe {
        __wasm_call_ctors();
    }
}

#[cfg(target_arch = "wasm32")]
pub fn main() {
    _initialize(); // As __wasm_call_ctors still does necessary work, we call it at the start of the module
    ed25519::main()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {}
