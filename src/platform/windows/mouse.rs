use std::convert::TryInto;
use std::mem::transmute;

use winapi::ctypes::c_int;
use winapi::shared::windef::POINT;
use winapi::um::winuser;

use super::sendinput_data;
use super::send_input;

use action::MouseButton;
use crate::Position;


/// # Win32 SendInput wrapper for mouse
fn send_mouse_event(mi: winuser::MOUSEINPUT) {
    let u = unsafe {
        transmute::<winuser::MOUSEINPUT, winuser::INPUT_u>(mi)
    };

    send_input(winuser::INPUT { type_: 0, u });
}


/// Moves mouse, using Win32 SetCursorPos() function
pub(super) fn mouse_move(p: Position) {
    let success = 0 != unsafe {
        winuser::SetCursorPos(p.x as c_int, p.y as c_int)
    };
    assert!(success, "Could not move mouse");
}


/// Mouse down
pub(super) fn mouse_down(button: MouseButton, p: Position) {
    let input_mask = match button {
        MouseButton::Left   => sendinput_data::MouseEventF::LEFTDOWN,
        MouseButton::Right  => sendinput_data::MouseEventF::RIGHTDOWN,
        // MouseButton::Middle => sendinput_data::MouseEventF::MIDDLEDOWN,
    };

    let mi = sendinput_data::new_mouseinput(p, input_mask.bits(), 0);
    send_mouse_event(mi)
}

/// Mouse up
pub(super) fn mouse_up(button: MouseButton, p: Position) {
    let input_mask = match button {
        MouseButton::Left   => sendinput_data::MouseEventF::LEFTUP,
        MouseButton::Right  => sendinput_data::MouseEventF::RIGHTUP,
        // MouseButton::Middle => sendinput_data::MouseEventF::MIDDLEUP,
    };

    let mi = sendinput_data::new_mouseinput(p, input_mask.bits(), 0);
    send_mouse_event(mi)
}

/// Mouse multiple clicks
pub(super) fn mouse_n_click(button: MouseButton, p: Position, n: u8) {
    for _ in 0..n {
        mouse_down(button, p);
        mouse_up(button, p);
    }
}

pub fn get_mouse_position() -> Option<Position> {
    let mut point = POINT {
        x: 0,
        y: 0
    };

    unsafe {
        if winuser::GetCursorPos(&mut point) == 1 {
            Some(Position::new(point.x.try_into().unwrap(), point.y.try_into().unwrap()))
        } else {
            None
        }
    }
}