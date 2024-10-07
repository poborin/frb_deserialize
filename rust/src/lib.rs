use flutter_rust_bridge::{frb, setup_default_user_utils};
use std::backtrace::Backtrace;
use std::cell::Cell;
use std::env;

pub mod api;
mod frb_generated;

thread_local! {
    static BACKTRACE: Cell<Option<Backtrace>> = const { Cell::new(None) };
}

#[frb(init)]
pub fn init_app() {
    env::set_var("RUST_BACKTRACE", "1");
    setup_default_user_utils();

    std::panic::set_hook(Box::new(|_| {
        let trace = Backtrace::capture();
        BACKTRACE.with(move |b| b.set(Some(trace)));
    }));
}