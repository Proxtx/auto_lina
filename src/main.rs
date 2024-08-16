use std::thread;

use active_win_pos_rs::get_active_window;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use rand::{seq::SliceRandom, Rng};

fn main() {
    let mut e = Enigo::new(&Settings::default()).unwrap();
    let keys = [
        Key::Unicode('w'),
        Key::Unicode('a'),
        Key::Unicode('s'),
        Key::Unicode('d'),
        Key::Space,
    ];
    let mut rng = rand::thread_rng();

    thread::spawn(move || {
        let mut e = Enigo::new(&Settings::default()).unwrap();
        loop {
            if let Ok(w) = get_active_window() {
                if w.app_name == "MosaLina" {
                    e.key(Key::Unicode('e'), Direction::Click).unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(10));
                    e.key(Key::Unicode('q'), Direction::Click).unwrap();
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let mut e = Enigo::new(&Settings::default()).unwrap();
        loop {
            if let Ok(w) = get_active_window() {
                if w.app_name == "MosaLina" {
                    e.key(Key::Unicode('r'), Direction::Click).unwrap();
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    });

    loop {
        if let Ok(w) = get_active_window() {
            if w.app_name == "MosaLina" {
                let key = *keys.choose(&mut rng).unwrap();
                e.key(key, Direction::Press).unwrap();
                std::thread::sleep(std::time::Duration::from_millis(rng.gen_range(0..=1000)));
                e.key(key, Direction::Release).unwrap();
            }
        };
        std::thread::sleep(std::time::Duration::from_millis(10))
    }
}
