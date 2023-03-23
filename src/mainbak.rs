use std::ptr::null_mut;
use winapi::shared::windef::{POINT, RECT};
use winapi::um::winuser::{
    CreateWindowExA, DefWindowProcA, GetMessageA, PostQuitMessage, RegisterClassA, ShowWindow,
    TranslateMessage, DispatchMessageA, MSG, WM_DESTROY, WNDCLASSA, SW_SHOW, CW_USEDEFAULT,
    WS_OVERLAPPEDWINDOW,
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

        // Create a window
        let hwnd = CreateWindowExA(
            0,
            class_name,
            "Hello World\0".as_ptr() as *const i8,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        );

        // Show the window
        ShowWindow(hwnd, SW_SHOW);

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
                // handle error
            } else {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }

        // Clean up
        PostQuitMessage(0);
    }
}