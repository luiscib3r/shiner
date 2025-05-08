mod frb_generated;

mod ai;
pub mod core;
pub mod data;

pub use libsql::Connection;

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
