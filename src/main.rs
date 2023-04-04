use windows::{Win32::{
    Foundation::*,
    UI::{WindowsAndMessaging::*,Controls::RichEdit::*},
	Graphics::Gdi::*,
	System::LibraryLoader::*,
},core::*,w};


/*
Foundation::{LPARAM, LRESULT, POINT, RECT, WPARAM,HWND},
UI::WindowsAndMessaging::{
        BS_DEFPUSHBUTTON, BS_PUSHBUTTON, CreateWindowExW, CW_USEDEFAULT,
        DefWindowProcW, DispatchMessageW, ES_AUTOHSCROLL, ES_LEFT, GCLP_HBRBACKGROUND, GetClientRect,
        */
use windows::Win32::UI::WindowsAndMessaging::WNDCLASSW;
use std::mem::{size_of, zeroed};

fn main() {
	
	// Get the HINSTANCE for the current process
let h_instance = unsafe { GetModuleHandleW(None).unwrap() };

// Load the default application icon
let h_icon = unsafe { LoadIconW(None, IDI_APPLICATION).unwrap() };

// Load the default arrow cursor
let h_cursor = unsafe { LoadCursorW(None, IDC_ARROW).unwrap() };

// Register a window class
//let class_name = "MyWindowClass\0".encode_utf16().collect::<Vec<u16>>();
let class_name = w!("MyWindowClass");
let wnd_class = WNDCLASSW {
    style: WNDCLASS_STYLES(0),
    lpfnWndProc: Some(window_proc),
    hInstance: h_instance,
    lpszClassName: windows::core::PCWSTR(class_name.as_ptr()),
    cbClsExtra: 0,
    cbWndExtra: 0,
    hIcon: h_icon,
    hCursor: h_cursor,
    hbrBackground: unsafe { GetSysColorBrush(COLOR_WINDOW) },
    lpszMenuName: PCWSTR::null(),
};
    unsafe {
       
        RegisterClassW(&wnd_class);

        // Create the main window
        let hwnd_main = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            class_name,
            w!("Hello World")
            WS_OVERLAPPEDWINDOW | WS_SYSMENU,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            640,
            480,
            HWND(0),
            HMENU(0),
            h_instance,
            None,
        );

        // Create the text box
        let hwnd_text = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("Edit"),
            w!(""),
            WS_CHILD | WS_VISIBLE | ES_LEFT | ES_AUTOHSCROLL,
            10,
            10,
            300,
            20,
            HWND(0),
            HMENU(0),
            h_instance,
            None,
        );

        // Create the OK button
        let hwnd_ok = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("Button"),
            w!("Ok"),
            WS_VISIBLE | WS_CHILD | BS_DEFPUSHBUTTON,
            320,
            10,
            80,
            20,
            HWND(0),
            HMENU(IDOK),
            h_instance,
            None,
        );

        // Create the Cancel button
        let hwnd_cancel = CreateWindowExW(
            0,
            Button::get_window_class_name().as_ptr(),
            "Cancel\0".encode_utf16().as_ptr(),
            WS_VISIBLE | WS_CHILD | BS_PUSHBUTTON,
            410,
            10,
            80,
            20,
            hwnd_main,
            IDCANCEL as isize as HMENU,
            h_instance,
            std::ptr::null_mut(),
        );

        // Show the main window
        ShowWindow(hwnd_main, SW_SHOW);

        // Run the message loop
        let mut msg = MSG::default();
        loop {
            let result = GetMessageW(&mut msg, HWND::NULL, 0, 0);
            if result == 0 {
                break;
            } else if result == -1 {
                //handle error
            } else {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        // Clean up
        PostQuitMessage(0);
    }
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            let h_instance = GetModuleHandleW(None);
            let h_brush = CreateSolidBrush(COLOR_WINDOW);
            SetClassLongPtrW(hwnd, GCLP_HBRBACKGROUND, h_brush.0 as isize);
            let hwnd_text = CreateWindowExW(
                0,
                Edit::get_window_class_name().as_ptr(),
                "".encode_utf16().as_ptr(),
                WS_CHILD | WS_VISIBLE | ES_LEFT | ES_AUTOHSCROLL,
                10,
                10,
                300,
                20,
                hwnd,
                HMENU::NULL,
                h_instance,
                std::ptr::null_mut(),
            );
            SendMessageW(hwnd_text, EM_SETBKGNDCOLOR, 0, COLOR_WINDOW as isize);
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            let rect = RECT {
                left: ps.rcPaint.left,
                top: ps.rcPaint.top,
                right: ps.rcPaint.right,
                bottom: ps.rcPaint.bottom,
            };
            FillRect(hdc, &rect, GetSysColorBrush(COLOR_WINDOW));
            EndPaint(hwnd, &ps);
        }
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
    }
    0
}
