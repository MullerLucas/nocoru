pub fn wait_for_end_of_universe() -> wasm_bindgen_futures::JsFuture {
    let promise = js_sys::Promise::new(&mut |_resolve, _reject| { });
    wasm_bindgen_futures::JsFuture::from(promise)
}
