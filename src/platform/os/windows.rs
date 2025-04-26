use crate::platform::window::WindowId;
use crate::platform::event::{Event, MouseButton};
use super::OsBackend;
use std::collections::HashMap;
use std::time::Instant;

// Static counter for all windows to use
static mut COUNTER: i32 = 0;

#[cfg(windows)]
use windows::{
    core::PCWSTR,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
};

pub struct WindowsBackend {
    next_window_id: usize,
    windows: HashMap<WindowId, Win32Window>,
    start_time: Instant,
    running: bool,
}

struct Win32Window {
    window_id: WindowId,
    #[cfg(windows)]
    hwnd: HWND,
    width: u32,
    height: u32,
    dpi_factor: f32,
    title: String,
}

impl WindowsBackend {
    pub fn new() -> Self {
        Self {
            next_window_id: 1,
            windows: HashMap::new(),
            start_time: Instant::now(),
            running: false,
        }
    }

    #[cfg(windows)]
    fn register_window_class(&self) -> u16 {
        unsafe {
            let h_instance = GetModuleHandleW(None).unwrap();

            // Use a static class name to avoid issues with string lifetime
            let class_name = "mixWindowClass";
            let class_name_w: Vec<u16> = class_name.encode_utf16().chain(std::iter::once(0)).collect();

            // We'll just try to register the class and handle any errors

            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
                lpfnWndProc: Some(Self::wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: h_instance.into(),
                hIcon: HICON(0),
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                hbrBackground: HBRUSH(COLOR_WINDOW.0 as isize),
                lpszMenuName: PCWSTR::null(),
                lpszClassName: PCWSTR(class_name_w.as_ptr()),
                hIconSm: HICON(0),
            };

            // Try to register the class
            let class_atom = RegisterClassExW(&wc);
            if class_atom == 0 {
                let error = GetLastError();

                // If the class is already registered, that's fine
                if error.0 == 1410 { // ERROR_CLASS_ALREADY_EXISTS
                    return 1; // Return a non-zero value to indicate success
                }

                panic!("Failed to register window class: error code {}", error.0);
            }
            class_atom
        }
    }

    #[cfg(windows)]
    unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            },
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);

                // Fill the window with a light blue background
                let brush = CreateSolidBrush(COLORREF(0x00FFFFCC)); // Light yellow (BGR format)
                let _ = FillRect(hdc, &ps.rcPaint, brush);
                let _ = DeleteObject(brush);

                // Draw a button
                let pen = CreatePen(PS_SOLID, 2, COLORREF(0x00004CAF)); // Green (BGR format)
                let old_pen = SelectObject(hdc, pen);

                let button_brush = CreateSolidBrush(COLORREF(0x00004CAF)); // Green (BGR format)
                let old_brush = SelectObject(hdc, button_brush);

                let _ = Rectangle(hdc, 300, 300, 500, 380);

                // Draw button text
                SetBkMode(hdc, TRANSPARENT);
                SetTextColor(hdc, COLORREF(0x00FFFFFF)); // White (BGR format)

                let text = "Increment";
                let text_w: Vec<u16> = text.encode_utf16().collect();

                let _ = TextOutW(hdc, 350, 330, &text_w);

                // Draw a label
                SetTextColor(hdc, COLORREF(0x00F32196)); // Blue (BGR format)

                let text = "Hello, mix!";
                let text_w: Vec<u16> = text.encode_utf16().collect();

                let _ = TextOutW(hdc, 300, 100, &text_w);

                // Draw a counter label
                SetTextColor(hdc, COLORREF(0x00000000)); // Black (BGR format)

                // Use the static counter
                let text = format!("Counter: {}", COUNTER);
                let text_w: Vec<u16> = text.encode_utf16().collect();

                let _ = TextOutW(hdc, 300, 200, &text_w);

                SelectObject(hdc, old_brush);
                SelectObject(hdc, old_pen);
                let _ = DeleteObject(pen);
                let _ = DeleteObject(button_brush);

                let _ = EndPaint(hwnd, &ps);
                LRESULT(0)
            },
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

impl OsBackend for WindowsBackend {
    fn init(&mut self) {
        #[cfg(windows)]
        {
            self.register_window_class();
        }
    }

    fn create_window(&mut self, title: &str, width: u32, height: u32) -> WindowId {
        let window_id = WindowId(self.next_window_id);
        self.next_window_id += 1;

        #[cfg(windows)]
        {
            unsafe {
                let h_instance = GetModuleHandleW(None).unwrap();

                let class_name = "mixWindowClass";
                let class_name_w: Vec<u16> = class_name.encode_utf16().chain(std::iter::once(0)).collect();

                let title_w: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();

                let style = WS_OVERLAPPEDWINDOW;

                // Calculate the window size based on the client area size
                let mut rect = RECT {
                    left: 0,
                    top: 0,
                    right: width as i32,
                    bottom: height as i32,
                };

                let _ = AdjustWindowRect(&mut rect, style, FALSE);

                // Make sure we have a valid window class
                let _ = self.register_window_class();

                let hwnd = CreateWindowExW(
                    WINDOW_EX_STYLE(0),
                    PCWSTR(class_name_w.as_ptr()),
                    PCWSTR(title_w.as_ptr()),
                    style,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    rect.right - rect.left,
                    rect.bottom - rect.top,
                    None,
                    None,
                    h_instance,
                    None,
                );

                if hwnd.0 == 0 {
                    let error = GetLastError();
                    panic!("Failed to create window: error code {}", error.0);
                }

                // No need to store a pointer to self anymore

                let win32_window = Win32Window {
                    window_id,
                    hwnd,
                    width,
                    height,
                    dpi_factor: 1.0,
                    title: title.to_string(),
                };

                self.windows.insert(window_id, win32_window);

                // Show the window
                let _ = ShowWindow(hwnd, SW_SHOW);
                let _ = UpdateWindow(hwnd);
            }
        }

        #[cfg(not(windows))]
        {
            let win32_window = Win32Window {
                window_id,
                width,
                height,
                dpi_factor: 1.0,
                title: title.to_string(),
            };

            self.windows.insert(window_id, win32_window);
        }

        window_id
    }

    fn process_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();

        #[cfg(windows)]
        {
            unsafe {
                let mut msg = MSG::default();

                while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);

                    match msg.message {
                        WM_QUIT => {
                            self.running = false;
                            events.push(Event::Shutdown);
                        },
                        WM_SIZE => {
                            // Find the window that was resized
                            for (window_id, window) in &mut self.windows {
                                if window.hwnd == msg.hwnd {
                                    let width = (msg.lParam.0 & 0xFFFF) as u32;
                                    let height = ((msg.lParam.0 >> 16) & 0xFFFF) as u32;

                                    window.width = width;
                                    window.height = height;

                                    events.push(Event::WindowResize {
                                        window_id: *window_id,
                                        width: width as f32,
                                        height: height as f32,
                                        dpi_factor: window.dpi_factor,
                                    });

                                    break;
                                }
                            }
                        },
                        WM_CLOSE => {
                            // Find the window that was closed
                            for (window_id, window) in &self.windows {
                                if window.hwnd == msg.hwnd {
                                    events.push(Event::WindowClose {
                                        window_id: *window_id,
                                    });
                                    break;
                                }
                            }
                        },
                        WM_LBUTTONDOWN => {
                            let x = (msg.lParam.0 & 0xFFFF) as f32;
                            let y = ((msg.lParam.0 >> 16) & 0xFFFF) as f32;

                            // Check if the click is on the button
                            if x >= 300.0 && x <= 500.0 && y >= 300.0 && y <= 380.0 {
                                // Increment the static counter
                                unsafe {
                                    COUNTER += 1;
                                }

                                // Invalidate the window to trigger a redraw
                                let _ = InvalidateRect(msg.hwnd, None, FALSE);
                            }

                            // Find the window
                            for (window_id, _) in &self.windows {
                                if msg.hwnd == msg.hwnd {
                                    events.push(Event::MouseDown {
                                        window_id: *window_id,
                                        x,
                                        y,
                                        button: MouseButton::Left,
                                    });
                                    break;
                                }
                            }
                        },
                        WM_LBUTTONUP => {
                            let x = (msg.lParam.0 & 0xFFFF) as f32;
                            let y = ((msg.lParam.0 >> 16) & 0xFFFF) as f32;

                            // Find the window
                            for (window_id, _) in &self.windows {
                                if msg.hwnd == msg.hwnd {
                                    events.push(Event::MouseUp {
                                        window_id: *window_id,
                                        x,
                                        y,
                                        button: MouseButton::Left,
                                    });
                                    break;
                                }
                            }
                        },
                        WM_MOUSEMOVE => {
                            let x = (msg.lParam.0 & 0xFFFF) as f32;
                            let y = ((msg.lParam.0 >> 16) & 0xFFFF) as f32;

                            // Find the window
                            for (window_id, _) in &self.windows {
                                if msg.hwnd == msg.hwnd {
                                    events.push(Event::MouseMove {
                                        window_id: *window_id,
                                        x,
                                        y,
                                    });
                                    break;
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        events
    }

    fn render(&mut self) {
        #[cfg(windows)]
        {
            for (_, window) in &self.windows {
                unsafe {
                    // Invalidate the window to trigger a WM_PAINT message
                    let _ = InvalidateRect(window.hwnd, None, FALSE);
                    let _ = UpdateWindow(window.hwnd);
                }
            }
        }
    }

    fn shutdown(&mut self) {
        #[cfg(windows)]
        {
            for (_, window) in &self.windows {
                unsafe {
                    let _ = DestroyWindow(window.hwnd);
                }
            }
        }
    }
}



