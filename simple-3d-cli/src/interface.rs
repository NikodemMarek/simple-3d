use simple_3d_core::types::{keys::Key, pixel::Pixel, screen::Screen};
use std::{
    io::{Read, Write, stdout},
    thread,
};
use termion::{async_stdin, raw::IntoRawMode};

const BRIGHTNESS_PIXELS: [char; 10] = ['@', '%', '#', '*', '+', '=', '-', ':', '.', ' '];
const FPS: u64 = 60;

pub struct CliInterface;
impl simple_3d_core::Interface for CliInterface {
    fn start<
        F: FnMut() -> Option<Screen> + Send + 'static,
        R: Fn(u32, u32) + Send + 'static,
        T: Fn() + Send + 'static,
        K: Fn() + Send + 'static,
    >(
        mut on_frame: F,
        on_resize: R,
        timers: Vec<(u64, T)>,
        keys: Vec<(Key, K)>,
    ) -> Self {
        let stdout = stdout();
        let mut stdout = stdout.into_raw_mode().unwrap();
        let mut stdin = async_stdin().bytes();

        let interval = 1000 / FPS;
        let mut time_passed = 0;
        thread::spawn(move || {
            loop {
                let (width, height) = Self::get_screen_size();
                on_resize(width, height);

                thread::sleep(std::time::Duration::from_millis(interval));
                time_passed += interval;

                for (timer, on_tick) in timers.iter() {
                    if time_passed % timer == 0 {
                        on_tick();
                    }
                }

                let pressed = match stdin.next() {
                    Some(Ok(b'j')) => Some(Key::ArrowDown),
                    Some(Ok(b'k')) => Some(Key::ArrowUp),
                    Some(Ok(b'h')) => Some(Key::ArrowLeft),
                    Some(Ok(b'l')) => Some(Key::ArrowRight),
                    _ => None,
                };
                if let Some(pressed) = pressed {
                    for (key, on_key) in keys.iter() {
                        if pressed == *key {
                            on_key();
                        }
                    }
                }

                if let Some(screen) = on_frame() {
                    draw(&mut stdout, &screen);
                } else {
                    return;
                }
            }
        });

        CliInterface
    }

    fn get_screen_size() -> (u32, u32) {
        let (width, height) = termion::terminal_size().unwrap();
        (width as u32, height as u32)
    }
    fn wait(self) {
        let mut stdin = async_stdin().bytes();
        loop {
            if let Some(Ok(b'q')) = stdin.next() {
                break;
            }
        }
    }
}

fn draw(out: &mut impl Write, screen: &Screen) {
    let pixels = screen.buffer().iter().map(map_pixel).collect::<String>();
    write!(out, "{}", termion::clear::All).unwrap();
    write!(out, "{}", termion::cursor::Hide).unwrap();
    write!(out, "{}", pixels).unwrap();
    out.flush().unwrap();
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
