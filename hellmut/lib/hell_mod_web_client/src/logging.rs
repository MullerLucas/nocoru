use wasm_bindgen::prelude::wasm_bindgen;
extern crate console_error_panic_hook;

pub fn init_logging() {
    // use std::panic;
    // panic::set_hook(Box::new(console_error_panic_hook::hook));

    console_error_panic_hook::set_once()
}

// ----------------------------------------------------------------------------

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn debug(txt: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(txt: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn info(txt: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn warn(txt: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(txt: &str);
}

#[macro_export]
macro_rules! console_debug {
    ($($t:tt)*) => {
        $crate::logging::debug(&format_args!($($t)*).to_string())
    };
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        $crate::logging::log(&format_args!($($t)*).to_string())
    };
}

#[macro_export]
macro_rules! console_info {
    ($($t:tt)*) => {
        $crate::logging::info(&format_args!($($t)*).to_string())
    };
}

#[macro_export]
macro_rules! console_warn {
    ($($t:tt)*) => {
        $crate::logging::warn(&format_args!($($t)*).to_string())
    };
}

#[macro_export]
macro_rules! console_error {
    ($($t:tt)*) => {
        $crate::logging::error(&format_args!($($t)*).to_string())
    };
}
