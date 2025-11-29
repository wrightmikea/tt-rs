//! Keyboard event handlers.

use std::cell::RefCell;
use std::rc::Rc;
use tt_rs_core::WidgetId;
use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

pub fn create_keydown(
    d: Rc<RefCell<Option<WidgetId>>>,
    p: Rc<RefCell<Option<usize>>>,
) -> Callback<web_sys::KeyboardEvent> {
    Callback::from(move |e: web_sys::KeyboardEvent| {
        if d.borrow().is_some() {
            if let Some(c) = e.key().chars().next().filter(|c| c.is_ascii_digit()) {
                *p.borrow_mut() = Some(c.to_digit(10).unwrap() as usize);
                e.prevent_default();
            }
        }
    })
}

pub fn setup_keydown_listener(on_keydown: Callback<web_sys::KeyboardEvent>) {
    use_effect_with((), move |_| {
        let window = web_sys::window().unwrap();
        let cb = Closure::wrap(
            Box::new(move |e: web_sys::KeyboardEvent| on_keydown.emit(e)) as Box<dyn FnMut(_)>,
        );
        window
            .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
            .unwrap();
        let w = window.clone();
        let c = cb;
        move || {
            let _ = w.remove_event_listener_with_callback("keydown", c.as_ref().unchecked_ref());
        }
    });
}
