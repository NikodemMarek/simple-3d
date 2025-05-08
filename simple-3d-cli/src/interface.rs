use std::thread::{self, sleep};

use simple_3d_core::types::screen::Screen;

pub struct CliInterface;
impl simple_3d_core::Interface for CliInterface {
    fn get_screen_size() -> (u32, u32) {
        let (width, height) = (100, 100);
        (width, height)
    }

    fn handle_resize<C: Fn(u32, u32) + 'static>(on_resize: C) {
        // TODO: Listen for terminal size changes
        let (width, height) = Self::get_screen_size();
        on_resize(width, height);
    }

    fn register_timer<C: FnMut() + Send + 'static>(interval: i32, mut on_tick: C) {
        thread::spawn(move || {
            loop {
                sleep(std::time::Duration::from_millis(interval as u64));
                on_tick();
            }
        });
    }

    fn handle_key_hold<C: Fn() + 'static>(key: &str, on_hold: C) {}

    fn start<C: FnMut() + Send + 'static>(on_frame: C) {
        Self::register_timer(20, on_frame);
    }

    fn draw(screen: &Screen) {}
}
