// ----------------------------------------------------------------------------
//  err-to-hell-err
// ----------------------------------------------------------------------------

use hell_core::error::{HellError, HellErrorKind};

pub trait ErrToWebHellErr<V> {
    fn to_web_hell_err(self) -> Result<V, HellError>;
}

impl<V> ErrToWebHellErr<V> for Result<V, wasm_bindgen::JsValue> {
    fn to_web_hell_err(self) -> Result<V, HellError> {
        self.map_err(|val| {
            // TODO: should probably not use fmt::Debug :shrug
            let content = format!("{:?}", val);
            HellError::from_msg(HellErrorKind::WebError, content)
        })
    }
}
