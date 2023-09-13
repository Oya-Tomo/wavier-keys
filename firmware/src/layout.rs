use super::keycodes::KeyCodes::*;
use crate::keycodes::KeyCodes;

pub const MODE_KEY_POS: [usize; 2] = [3, 14]; // switch keyboard mode or mouse mode

pub const FN_KEY_POS: [usize; 2] = [2, 14]; // row, col

pub const LEFT_CTRL: [usize; 2] = [4, 0];
pub const LEFT_SHIFT: [usize; 2] = [3, 0];
pub const LEFT_ALT: [usize; 2] = [4, 2];
pub const LEFT_GUI: [usize; 2] = [4, 1];
pub const RIGHT_CTRL: [usize; 2] = [4, 10];
pub const RIGHT_SHIFT: [usize; 2] = [3, 11];
pub const RIGHT_ALT: [usize; 2] = [4, 8];
pub const RIGHT_GUI: [usize; 2] = [4, 9]; // unused in the layout

pub const MOVE_LEFT: [usize; 2] = [2, 6];
pub const MOVE_DOWN: [usize; 2] = [2, 7];
pub const MOVE_UP: [usize; 2] = [2, 8];
pub const MOVE_RIGHT: [usize; 2] = [2, 9];
pub const LEFT_BUTTON: [usize; 2] = [2, 10];
pub const RIGHT_BUTTON: [usize; 2] = [2, 11];

pub const KEY_LAYOUT: [[KeyCodes; 15]; 5] = [
    [
        Escape, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0, Minus, Equal, Grave,
        BackSpace,
    ],
    [
        Tab, KeyQ, KeyW, KeyE, KeyR, KeyT, KeyY, KeyU, KeyI, KeyO, KeyP, LeftBrace, RightBrace,
        BackSlash, Delete,
    ],
    [
        CapsLock, KeyA, KeyS, KeyD, KeyF, KeyG, KeyH, KeyJ, KeyK, KeyL, SemiColon, Apostrophe,
        Reserved, Enter, Reserved,
    ],
    [
        Reserved, KeyZ, KeyX, KeyC, KeyV, KeyB, KeyN, KeyM, Comma, Dot, Slash, Reserved, Reserved,
        Up, Reserved,
    ],
    [
        Reserved, Reserved, Reserved, Reserved, Reserved, Space, Reserved, Reserved, Reserved,
        Reserved, Reserved, Left, Reserved, Down, Right,
    ],
];

pub const KEY_LAYOUT_WITH_FN: [[KeyCodes; 15]; 5] = [
    [
        Escape, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, BackSpace,
    ],
    [
        Tab, KeyQ, KeyW, KeyE, KeyR, KeyT, KeyY, KeyU, KeyI, KeyO, KeyP, LeftBrace, RightBrace,
        BackSlash, Delete,
    ],
    [
        CapsLock, KeyA, KeyS, KeyD, KeyF, KeyG, KeyH, KeyJ, KeyK, KeyL, SemiColon, Apostrophe,
        Reserved, Enter, Reserved,
    ],
    [
        Reserved, KeyZ, KeyX, KeyC, KeyV, KeyB, KeyN, KeyM, Comma, Dot, Slash, Reserved, Reserved,
        Up, Reserved,
    ],
    [
        Reserved, Reserved, Reserved, Reserved, Reserved, Space, Reserved, Reserved, Reserved,
        Reserved, Reserved, Left, Reserved, Down, Right,
    ],
];

#[allow(unused)]
pub const TEST_KEY_LAYOUT: [[KeyCodes; 15]; 5] = [
    [
        KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA,
    ],
    [
        KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA,
    ],
    [
        KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA,
    ],
    [
        KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA,
    ],
    [
        KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA, KeyA,
    ],
];
