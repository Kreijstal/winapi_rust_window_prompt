use std::ptr::null_mut;
use winapi::shared::{windef::{POINT, RECT},
minwindef::{HIWORD,LOWORD}};

use winapi::um::winuser::{
    CreateWindowExA, DefWindowProcA, GetMessageA, PostQuitMessage, RegisterClassA, ShowWindow,
    TranslateMessage, DispatchMessageA, MSG, WM_DESTROY, WNDCLASSA, SW_SHOW, CW_USEDEFAULT,
    WS_OVERLAPPEDWINDOW, WS_VISIBLE, WS_CHILD, ES_AUTOHSCROLL, BS_DEFPUSHBUTTON, BS_PUSHBUTTON,
    IDCANCEL, IDOK, WM_COMMAND,
};
fn main() {
    unsafe {
        let class_name = "MyWindowClass\0".as_ptr() as *const i8;

        // Register a window class
        let wnd_class = WNDCLASSA {
            style: 0,
            lpfnWndProc: Some(DefWindowProcA),
            hInstance: null_mut(),
            lpszClassName: class_name,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
        };
        RegisterClassA(&wnd_class);

        // Create the main window
        let hwnd_main = CreateWindowExA(
            0,
            class_name,
            "Hello World\0".as_ptr() as *const i8,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            640,
            480,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        );

        // Create the text box
        let hwnd_text = CreateWindowExA(
            0,
            "EDIT\0".as_ptr() as *const i8,
            null_mut(),
            WS_VISIBLE | WS_CHILD | ES_AUTOHSCROLL,
            10,
            10,
            300,
            20,
            hwnd_main,
            null_mut(),
            null_mut(),
            null_mut(),
        );

        // Create the OK button
        let hwnd_ok = CreateWindowExA(
            0,
            "BUTTON\0".as_ptr() as *const i8,
            "OK\0".as_ptr() as *const i8,
            WS_VISIBLE | WS_CHILD | BS_DEFPUSHBUTTON,
            320,
            10,
            80,
            20,
            hwnd_main,
            null_mut(),
            null_mut(),
            null_mut(),
        );

        // Create the Cancel button
        let hwnd_cancel = CreateWindowExA(
            0,
            "BUTTON\0".as_ptr() as *const i8,
            "Cancel\0".as_ptr() as *const i8,
            WS_VISIBLE | WS_CHILD | BS_PUSHBUTTON,
            410,
            10,
            80,
            20,
            hwnd_main,
            null_mut(),
            null_mut(),
            null_mut(),
        );

        // Show the main window
        ShowWindow(hwnd_main, SW_SHOW);

        // Run the message loop
        let mut msg = MSG {
            hwnd: null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };
        loop {
            let result = GetMessageA(&mut msg, null_mut(), 0, 0);
            if result == 0 {
                break;
            } else if result == -1 {
                //handle error
            } else {
            if msg.message == WM_COMMAND {
                let id = LOWORD(msg.wParam as u32) as usize;
                let code = HIWORD(msg.wParam as u32);
                let hwnd = msg.hwnd;

                if code == 0 && id == IDOK as usize && hwnd == hwnd_main {
                    // OK button clicked
                    let mut text_buf = [0u8; 1024];
                    let text_len = winapi::um::winuser::GetWindowTextA(
                        hwnd_text,
                        text_buf.as_mut_ptr() as *mut i8,
                        text_buf.len() as i32,
                    );
                    let text = std::str::from_utf8_unchecked(&text_buf[..text_len as usize]);
                    println!("OK clicked with text: {}", text);
                    break;
                } else if code == 0 && id == IDCANCEL as usize && hwnd == hwnd_main {
                    // Cancel button clicked
                    println!("Cancel clicked");
                    break;
                }
            } else {
                    TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                }
            }
        }

        // Clean up
        PostQuitMessage(0);
    }
}
