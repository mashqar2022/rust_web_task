use super::AppState;
use std::sync::Mutex;

pub mod group_controller;
pub mod user_controller;

pub use group_controller::init as init_group_controller;
pub use user_controller::init as init_user_controller;

fn log_request(route: &'static str, connections: &Mutex<u32>) {
    let mut con = connections.lock().unwrap();
    *con += 1;
}
