#[cfg(not(target_arch = "wasm32"))]
mod wasmic_wasmer;

#[cfg(not(target_arch = "wasm32"))]
pub use wasmic_wasmer::*;

#[cfg(target_arch = "wasm32")]
mod wasmic_browser;

#[cfg(target_arch = "wasm32")]
pub use wasmic_browser::*;
