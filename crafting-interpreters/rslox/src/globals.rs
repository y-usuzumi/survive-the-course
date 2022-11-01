/*!
 * Module for global state.
 */

use std::sync::{Arc, Once, RwLock};
pub struct Globals {
    pub has_error: bool,
}

impl Globals {
    fn new() -> Self {
        Globals { has_error: false }
    }
}

static mut GLOBALS: Option<Arc<RwLock<Globals>>> = None;
static INIT: Once = Once::new();

pub fn init_globals() {
    INIT.call_once(|| unsafe { GLOBALS = Some(Arc::new(RwLock::new(Globals::new()))) })
}

pub fn globals<'a>() -> Arc<RwLock<Globals>> {
    unsafe { GLOBALS.clone().unwrap() }
}

pub fn set_has_error() {
    let globals = globals();
    let mut globals_write_guard = globals.write().unwrap();
    (*globals_write_guard).has_error = true;
}
