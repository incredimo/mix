use crate::window::WindowId;

#[derive(Clone, Debug)]
pub enum Event {
    None,
    Init,
    Shutdown,
    Draw,

    WindowResize {
        window_id: WindowId,
        width: f32,
        height: f32,
        dpi_factor: f32,
    },

    WindowClose {
        window_id: WindowId,
    },

    MouseDown {
        window_id: WindowId,
        x: f32,
        y: f32,
        button: MouseButton,
    },

    MouseUp {
        window_id: WindowId,
        x: f32,
        y: f32,
        button: MouseButton,
    },

    MouseMove {
        window_id: WindowId,
        x: f32,
        y: f32,
    },

    KeyDown {
        window_id: WindowId,
        key_code: KeyCode,
        is_repeat: bool,
    },

    KeyUp {
        window_id: WindowId,
        key_code: KeyCode,
    },

    TextInput {
        window_id: WindowId,
        input: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyCode {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Left,
    Up,
    Right,
    Down,
    Backspace,
    Return,
    Space,
    Tab,
    Shift,
    Control,
    Alt,
    CapsLock,
    NumLock,
    ScrollLock,
    Unknown,
}

pub trait EventHandler {
    fn handle_event(&mut self, cx: &mut crate::cx::Cx, event: &Event);
}
