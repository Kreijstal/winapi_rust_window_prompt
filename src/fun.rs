use windows::{
    core::*,
    w,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::{LibraryLoader::*, WindowsProgramming::*},
        UI::{Controls::RichEdit::*, WindowsAndMessaging::*,Input::KeyboardAndMouse::*,Shell::*},
    },
};
use once_cell::sync::Lazy;
use std::sync::Mutex;
//use serde::{Deserialize, Serialize};
mod savewindow;
use std::collections::HashMap;
static RESULTS: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
static WINDOWHANDLES: Lazy<Mutex<HashMap<String,HWND>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn LOWORD(l: usize) -> usize {
    l & 0xffff
}

pub fn HIWORD(l: usize) -> usize {
    (l >> 16) & 0xffff
}
fn get_text_from_textbox(hwnd: HWND)->(){
	// Get the length of the text
                let text_length = unsafe{GetWindowTextLengthW(hwnd) };

                // Allocate a buffer to store the text
                let mut text_buffer: Vec<u16> = vec![0; (text_length + 1) as usize];

                // Retrieve the text
                unsafe{GetWindowTextW(hwnd, &mut text_buffer);}

                // Convert the text to a Rust String
                let text = String::from_utf16_lossy(&text_buffer);
				let text = text.trim_matches(char::from(0)).to_string();

                // Use the text as needed
				let mut te=RESULTS.lock().unwrap();
                *te=Some(text);
}
unsafe extern "system" fn text_box_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    _id_subclass: usize,
    _ref_data: usize,
) -> LRESULT {
    match msg {
        WM_KEYDOWN => {
            if wparam.0 == VK_RETURN.0 as usize{
                // Enter key has been pressed, handle the event here
                // ...
				get_text_from_textbox(hwnd);
				
				
				PostQuitMessage(0);

                // You can return 0 to prevent the Enter key from being processed further
                return LRESULT(0);
				
            }
        }
        _ => {}
    }

    // Call the original window procedure (DefSubclassProc)
    DefSubclassProc(hwnd, msg, wparam, lparam)
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
		WM_LBUTTONDOWN=>{
			PostMessageW(hwnd,WM_SYSCOMMAND,WPARAM((SC_SIZE as usize)+9),LPARAM(0));
		}
        WM_CREATE => {
            // Get the HINSTANCE for the current process
			SetWindowLongW(hwnd, GWL_STYLE, 0); //remove all window styles, check MSDN for details
            let h_instance = unsafe { GetModuleHandleW(None).unwrap() };
            // Create the text box
            let hwnd_text = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                w!("EDIT"),
                w!(""),
                WS_CHILD
                    | WS_VISIBLE
                    | WS_BORDER
                    | WINDOW_STYLE(ES_LEFT as u32)
                    | WINDOW_STYLE(ES_AUTOHSCROLL as u32),
                10,
                10,
                300,
                20,
                hwnd,
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
                hwnd,
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
                hwnd,
                HMENU(IDCANCEL.0 as isize),
                h_instance,
                None,
            );
			let mut winhand=WINDOWHANDLES.lock().unwrap();
			winhand.insert("Ok".to_string(), hwnd_ok);//["Ok"]=hwnd_ok;
			winhand.insert("Cancel".to_string(), hwnd_cancel);
			winhand.insert("Edit".to_string(), hwnd_text);
			//winhand["Cancel"]=hwnd_cancel;
			//winhand["Edit"]=hwnd_text;
			
			// Setting fonts.
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
                let hdc = unsafe { GetDC(hwnd) };
                let font_size = -MulDiv(8, GetDeviceCaps(hdc, LOGPIXELSY), 72);
                unsafe {
                    ReleaseDC(hwnd, hdc);
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
                SetClassLongPtrW(hwnd, GCLP_HBRBACKGROUND, hbr.0);

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
			savewindow::load_window_placement(hwnd,"window.txt");
			SetWindowSubclass(
                hwnd_text,
                Some(text_box_proc),
                0, // id_subclass, can be any value
                0, // ref_data, not used in this example
            );
        }
        WM_COMMAND => {
            let control_id = LOWORD(wparam.0);
            let notification_code = HIWORD(wparam.0);
            let control_handle = lparam;

            if control_id == IDOK.0 as usize
                && notification_code == 0
                && control_handle.0 == GetDlgItem(hwnd, IDOK.0).0
            {
                println!("OK button clicked!");
				let mut winhand=WINDOWHANDLES.lock().unwrap();
				get_text_from_textbox(winhand["Edit"]);
				PostQuitMessage(0);
            } else if control_id == IDCANCEL.0 as usize
                && notification_code == 0
                && control_handle.0 == GetDlgItem(hwnd, IDCANCEL.0).0
            {
                println!("Cancel button clicked!");
				savewindow::save_window_placement(hwnd,"window.txt");
				PostQuitMessage(0);
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

pub fn create_window()->Option<String> {
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
            w!("There should not be a visible title :p"),
            WS_OVERLAPPEDWINDOW | WS_SYSMENU,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            500,
            40,
            HWND(0),
            HMENU(0),
            h_instance,
            None,
        );

        // Show the main window
        ShowWindow(hwnd_main, SW_SHOW);

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
		let mut winhand=WINDOWHANDLES.lock().unwrap();
		*winhand=HashMap::new();
        PostQuitMessage(0);
		
		let texxtg=RESULTS.lock().unwrap();
		let text=texxtg.as_ref();
		//let text=*texxtg;
		text.cloned()
    }
}
