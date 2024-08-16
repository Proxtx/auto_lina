use active_win_pos_rs::get_active_window;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};

fn main() {
    let mut e = Enigo::new(&Settings::default()).unwrap();
    loop {
        if let Ok(w) = get_active_window() {
            println!("{}", w.app_name);
            if w.app_name == "Mosa Lina" {
                println!(" OK{}", w.app_name);
                e.key(Key::E, Direction::Click).unwrap();
                //simulate::press(simulate::Key::E).unwrap();
            }
        };
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}
