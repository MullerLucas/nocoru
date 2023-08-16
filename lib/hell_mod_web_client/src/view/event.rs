use hell_core::error::HellResult;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Event;

use crate::{error::ErrToWebHellErr, console_error};

// ---------------------------------------------------------------------------  -

pub struct EventHandlerId(usize);

impl EventHandlerId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
pub struct EventHandler {
    closure: Closure<dyn FnMut(Event)>
}

impl EventHandler {
    pub fn from_event<C>(element: &web_sys::Element, event_type: &str, cb: C) -> HellResult<Self>
    where C: FnMut(Event) + 'static
    {
        let closure = Closure::wrap(Box::new(cb) as Box<dyn FnMut(Event)>);

        element
            .add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())
            .to_web_hell_err()
            .map_err(|e| {
                console_error!("failed to add event listener: {:?}", e);
                e
            })?;

        Ok(Self {
            closure,
        })
    }

    pub fn closure_function(&self) -> &js_sys::Function {
        self.closure.as_ref().unchecked_ref()
    }

    // NOTE: leaks memory
    pub fn forget(self) {
        self.closure.forget()
    }
}
