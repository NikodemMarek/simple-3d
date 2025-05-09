use std::io::{Read, Write};
use std::thread::JoinHandle;
use std::{io::stdout, thread};

use simple_3d_core::{
    Guard,
    types::{pixel::Pixel, screen::Screen},
};
use termion::screen::IntoAlternateScreen;

const BRIGHTNESS_PIXELS: [char; 10] = ['@', '%', '#', '*', '+', '=', '-', ':', '.', ' '];

pub struct CliInterface;
impl simple_3d_core::Interface for CliInterface {
    fn get_screen_size() -> (u32, u32) {
        let (width, height) = termion::terminal_size().unwrap();
        (width as u32, height as u32)
    }

    fn handle_resize<C: Fn(u32, u32) + 'static>(on_resize: C) -> impl Guard {
        let (width, height) = Self::get_screen_size();
        on_resize(width, height);
        CliEmptyGuard
    }

    fn register_timer<C: FnMut() + Send + 'static>(interval: i32, mut on_tick: C) -> impl Guard {
        let handle = thread::spawn(move || {
            loop {
                thread::sleep(std::time::Duration::from_millis(interval as u64));
                on_tick();
            }
        });
        CliThreadGuard(handle)
    }

    fn handle_key_hold<C: Fn() + Send + 'static>(key: &str, on_hold: C) -> impl Guard {
        let mut input = termion::async_stdin();
        let key = key.to_string();
        let handle = thread::spawn(move || {
            let mut buffer = [0; 1];
            loop {
                if input.read_exact(&mut buffer).is_ok() && buffer[0] == key.as_bytes()[0] {
                    on_hold();
                }
            }
        });
        CliThreadGuard(handle)
    }

    fn start<C: FnMut() + Send + 'static>(on_frame: C) -> impl Guard {
        Self::register_timer(20, on_frame)
    }

    fn draw(screen: &Screen) {
        let mut out = stdout().into_alternate_screen().unwrap();
        // write!(out, "{}", termion::clear::All).unwrap();
        let pixels = screen.buffer().iter().map(map_pixel).collect::<String>();
        write!(out, "{}", termion::cursor::Hide).unwrap();
        write!(out, "{}", pixels).unwrap();
        out.flush().unwrap();
    }
}

fn map_pixel(pixel: &Pixel) -> String {
    use termion::color;
    let value = match pixel.brightness() {
        0.0..=25.0 => BRIGHTNESS_PIXELS[0],
        25.0..=50.0 => BRIGHTNESS_PIXELS[1],
        50.0..=75.0 => BRIGHTNESS_PIXELS[2],
        75.0..=100.0 => BRIGHTNESS_PIXELS[3],
        100.0..=125.0 => BRIGHTNESS_PIXELS[4],
        125.0..=150.0 => BRIGHTNESS_PIXELS[5],
        150.0..=175.0 => BRIGHTNESS_PIXELS[6],
        175.0..=200.0 => BRIGHTNESS_PIXELS[7],
        200.0..=225.0 => BRIGHTNESS_PIXELS[8],
        225.0..=255.0 => BRIGHTNESS_PIXELS[9],
        _ => BRIGHTNESS_PIXELS[9],
    };
    format!(
        "{}{}",
        color::Fg(color::Rgb(pixel.0, pixel.1, pixel.2)),
        value
    )
}

pub struct CliThreadGuard(JoinHandle<u8>);
impl Guard for CliThreadGuard {
    fn is_finished(&self) -> bool {
        self.0.is_finished()
    }
    fn stop(self) {
        self.0.thread().unpark();
    }
}

pub struct CliEmptyGuard;
impl Guard for CliEmptyGuard {
    fn is_finished(&self) -> bool {
        false
    }
    fn stop(self) {}
}
