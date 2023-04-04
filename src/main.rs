use std::ptr::null_mut;
use winapi::shared::{
    minwindef::{HIWORD, LOWORD, LPARAM, LRESULT, UINT, WPARAM},
    windef::{HWND, POINT, RECT},
};

use std::ffi::CString;
use winapi::shared::basetsd::LONG_PTR;
use winapi::um::wingdi::GetStockObject;
use winapi::um::wingdi::SYSTEM_FONT;
use winapi::um::winuser::GetSysColor;
use winapi::um::winuser::COLOR_WINDOWTEXT;
use winapi::um::winuser::GetDlgItem;
use winapi::um::winuser::GetSysColorBrush;
use winapi::um::winuser::SendMessageA;
use winapi::um::winuser::SetClassLongPtrA;
use winapi::um::winuser::COLOR_WINDOW;
use winapi::um::winuser::GCLP_HBRBACKGROUND;
use winapi::um::winuser::WM_SETFONT;

use winapi::um::winuser::{
    CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, PostQuitMessage,
    RegisterClassA, ShowWindow, TranslateMessage, BS_DEFPUSHBUTTON, BS_PUSHBUTTON, CW_USEDEFAULT,
    ES_AUTOHSCROLL, IDCANCEL, IDOK, MSG, SW_SHOW, WM_COMMAND, WM_DESTROY, WNDCLASSA, WS_CHILD,
    WS_OVERLAPPEDWINDOW, WS_SYSMENU, WS_VISIBLE,
};
extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_COMMAND => {
            let id = LOWORD(wparam as u32) as usize;
            let code = HIWORD(wparam as u32);
            let control_hwnd = lparam as HWND;
            if code == 0 && id == IDOK as usize && control_hwnd == unsafe { GetDlgItem(hwnd, IDOK) }
            {
                println!("Ok clicked!");
            } else if code == 0
                && id == IDCANCEL as usize
                && control_hwnd == unsafe { GetDlgItem(hwnd, IDCANCEL) }
            {
                println!("Cancel Clicked");
            }
            0
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            0
        }
        _ => unsafe { DefWindowProcA(hwnd, msg, wparam, lparam) },
    }
}
fn main() {
    unsafe {
        let class_name = "MyWindowClass\0".as_ptr() as *const i8;
        //let class_name = CString::new("MyWindowClass").unwrap();
        // Register a window class
        let wnd_class = WNDCLASSA {
            style: 0,
            lpfnWndProc: Some(window_proc), //Some(DefWindowProcA),
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
            WS_OVERLAPPEDWINDOW | WS_SYSMENU,
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
            IDOK as isize as *mut _,
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
            IDCANCEL as isize as *mut _,
            null_mut(),
            null_mut(),
        );
       /* let hfont = GetStockObject(SYSTEM_FONT);
        SendMessageA(hwnd_text, WM_SETFONT, hfont as WPARAM, 0);
        let hbr = GetSysColorBrush(COLOR_WINDOW);
        SetClassLongPtrA(hwnd_main, GCLP_HBRBACKGROUND, hbr as LONG_PTR);
        SendMessageA(hwnd_text, EM_SETBKGNDCOLOR, 0, COLOR_WINDOW as LPARAM);

        let hfont = GetStockObject(SYSTEM_FONT);
        SendMessageA(hwnd_text, WM_SETFONT, hfont as WPARAM, 0);

        let hcolor = GetSysColor(COLOR_WINDOWTEXT);
        SendMessageA(hwnd_text, EM_SETTEXTCOLOR, hcolor as WPARAM, 0);
*/
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
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }

        // Clean up
        PostQuitMessage(0);
    }
}
