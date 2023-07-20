use std::ffi::OsStr;

use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::TRUE;
use winapi::shared::windef::{HBRUSH, HDC, HWND, RECT};
use winapi::um::wingdi::{GetTextExtentPoint32W, TextOutW};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, GetClientRect,
    GetMessageW, LoadCursorW, PostQuitMessage, RegisterClassW, ShowWindow, TranslateMessage,
    COLOR_WINDOW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, MSG, PAINTSTRUCT, SW_SHOW,
    WM_CREATE, WM_DESTROY, WM_PAINT, WNDCLASSW, WS_OVERLAPPEDWINDOW,
};

use std::cell::RefCell;
thread_local!(static INPUT_STRING: RefCell<String> = RefCell::new("".into()));

fn add_customized_text(s: &str) {
    INPUT_STRING.with(|data| {
        *data.borrow_mut() = s.into();
    })
}

fn get_customized_text() -> String {
    INPUT_STRING.with(|data| data.borrow().to_owned())
}

pub fn create_window(title: &str, text: &str) -> HWND {
    unsafe {
        let class_name = format!("{}_class\0", title)
            .encode_utf16()
            .collect::<Vec<u16>>();
        let window_name = format!("{}\0", title).encode_utf16().collect::<Vec<u16>>();

        let h_instance = winapi::um::libloaderapi::GetModuleHandleW(std::ptr::null());

        add_customized_text(text);

        let wnd_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: h_instance,
            lpszClassName: class_name.as_ptr(),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: std::ptr::null_mut(),
            hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
            hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
            lpszMenuName: std::ptr::null_mut(),
        };

        RegisterClassW(&wnd_class);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            h_instance,
            std::ptr::null_mut(),
        );

        ShowWindow(hwnd, SW_SHOW);

        let mut msg: MSG = std::mem::zeroed();

        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        hwnd
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            return TRUE as LRESULT;
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            return TRUE as LRESULT;
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc: HDC = BeginPaint(hwnd, &mut ps);
            let text_wide: Vec<u16> = OsStr::new(&get_customized_text())
                .to_string_lossy()
                .encode_utf16()
                .collect();
            let text_ptr = text_wide.as_ptr();
            let text_len = text_wide.len() as i32;

            let mut rect: RECT = std::mem::zeroed();
            GetClientRect(hwnd, &mut rect);

            let mut text_size = std::mem::zeroed();
            GetTextExtentPoint32W(hdc, text_ptr, text_len, &mut text_size);

            let x = (rect.right - text_size.cx) / 2;
            let y = (rect.bottom - text_size.cy) / 2;

            TextOutW(hdc, x, y, text_ptr, text_len);

            EndPaint(hwnd, &ps);
            return TRUE as LRESULT;
        }
        _ => (),
    }

    DefWindowProcW(hwnd, msg, w_param, l_param)
}
