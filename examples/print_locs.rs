extern crate autogui;

use std::time::Duration;
use std::thread::sleep;

use autogui::{AutoGUI, Position};


fn main() {
    let gui = AutoGUI::new();

    println!("Move to (0, 0) to exit.");

    loop {
        let Position { x, y } = gui.mouse.get_position();
        if x == 0 && y == 0 {
            break
        } else {
            println!("x={}, y={}", x, y);
            sleep(Duration::from_millis(500));
        }
    }
}