use windows::{
    core::*,
    w,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::{LibraryLoader::*, WindowsProgramming::*},
        UI::{Controls::RichEdit::*, WindowsAndMessaging::*},
    },
};

pub fn LOWORD(l: usize) -> usize {
    l & 0xffff
}

pub fn HIWORD(l: usize) -> usize {
    (l >> 16) & 0xffff
}
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_COMMAND => {
            let control_id = LOWORD(wparam.0);
            let notification_code = HIWORD(wparam.0);
            let control_handle = lparam;

            if control_id == IDOK.0 as usize
                && notification_code == 0
                && control_handle.0 == GetDlgItem(hwnd, IDOK.0).0
            {
                println!("OK button clicked!");
            } else if control_id == IDCANCEL.0 as usize
                && notification_code == 0
                && control_handle.0 == GetDlgItem(hwnd, IDCANCEL.0).0
            {
                println!("Cancel button clicked!");
            }
        }
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => {
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
    }

    return LRESULT(0);
}

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
            w!("Only 1 letter in title"),
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
            w!("EDIT"),
            w!(""),
            WS_CHILD
                | WS_VISIBLE
                | WINDOW_STYLE(ES_LEFT as u32)
                | WINDOW_STYLE(ES_AUTOHSCROLL as u32),
            10,
            10,
            300,
            20,
            hwnd_main,
            HMENU(0),
            h_instance,
            None,
        );

        // Create the OK button
        let hwnd_ok = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("BUTTON"),
            w!("Ok"),
            WS_VISIBLE | WS_CHILD | WINDOW_STYLE(BS_DEFPUSHBUTTON as u32),
            320,
            10,
            80,
            20,
            hwnd_main,
            HMENU(IDOK.0 as isize),
            h_instance,
            None,
        );

        // Create the Cancel button
        let hwnd_cancel = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("BUTTON"),
            w!("Cancel"),
            WS_VISIBLE | WS_CHILD | WINDOW_STYLE(BS_DEFPUSHBUTTON as u32),
            410,
            10,
            80,
            20,
            hwnd_main,
            HMENU(IDCANCEL.0 as isize),
            h_instance,
            None,
        );

        // Show the main window
        ShowWindow(hwnd_main, SW_SHOW);
        // Retrieve a handle to the system color brush for the window background
        let hbr = unsafe { GetSysColorBrush(COLOR_WINDOW) };
        let mut non_client_metrics = NONCLIENTMETRICSW::default();
        non_client_metrics.cbSize = std::mem::size_of::<NONCLIENTMETRICSW>() as u32;
        let result = unsafe {
            use core::ffi::c_void;
            SystemParametersInfoW(
                SPI_GETNONCLIENTMETRICS,
                non_client_metrics.cbSize,
                Some(&mut non_client_metrics as *mut _ as *mut c_void),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
            )
        };
        let message_log_font = if result.as_bool() {
            let message_log_font: LOGFONTW = non_client_metrics.lfMessageFont;
            // You now have the LOGFONTW for the current font in message_log_font.
            message_log_font
        } else {
            // Handle the error case.
            // Define the font name and size
            let font_name = w!("Tahoma");
            let hdc = unsafe { GetDC(hwnd_main) };
            let font_size = -MulDiv(8, GetDeviceCaps(hdc, LOGPIXELSY), 72);
            unsafe {
                ReleaseDC(hwnd_main, hdc);
            };
            // Create a new LOGFONTW struct for the font
            let mut log_font = LOGFONTW::default();
            log_font.lfHeight = font_size;
            unsafe {
                std::ptr::copy_nonoverlapping(
                    font_name.0,
                    log_font.lfFaceName.as_mut_ptr(),
                    log_font.lfFaceName.len().min(wcslen(font_name)),
                );
            }
            // Create a handle to the font
            log_font
        };
        let hfont = unsafe { CreateFontIndirectW(&message_log_font) };

        // Retrieve a handle to the system font
        //let hfont = unsafe { GetStockObject(SYSTEM_FONT) };

        // Retrieve the system color for the window text
        let hcolor = unsafe { GetSysColor(COLOR_WINDOWTEXT) };

        unsafe {
            // Set the background color for the main window class
            SetClassLongPtrW(hwnd_main, GCLP_HBRBACKGROUND, hbr.0);

            SendMessageW(hwnd_text, WM_SETFONT, WPARAM(hfont.0 as usize), LPARAM(0));
            SendMessageW(hwnd_ok, WM_SETFONT, WPARAM(hfont.0 as usize), LPARAM(0));
            SendMessageW(hwnd_cancel, WM_SETFONT, WPARAM(hfont.0 as usize), LPARAM(0));

            // Set the background color for the text control
            SendMessageW(
                hwnd_text,
                EM_SETBKGNDCOLOR,
                WPARAM(COLOR_WINDOW.0 as usize),
                LPARAM(hbr.0),
            );
        }

        // Run the message loop
        let mut msg = MSG::default();
        loop {
            let result = GetMessageW(&mut msg, HWND(0), 0, 0);
            if result == BOOL(0) {
                break;
            } else if result == BOOL(-1) {
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
