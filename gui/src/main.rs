#![windows_subsystem = "windows"]
#![forbid(unsafe_code)]

mod app;
mod style;

fn main() {
    crate::app::start();
}
