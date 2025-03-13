use serde::{Deserialize, Serialize};
use windows::Win32::UI::WindowsAndMessaging::WINDOWPLACEMENT;
use windows::Win32::Foundation::POINT;
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD;
use windows::Win32::UI::WindowsAndMessaging::WINDOWPLACEMENT_FLAGS;
use windows::Win32::UI::WindowsAndMessaging::GetWindowPlacement;
use windows::Win32::UI::WindowsAndMessaging::SetWindowPlacement;
use windows::Win32::Foundation::HWND;
#[derive(Serialize, Deserialize)]
pub struct SerializableWINDOWPLACEMENT {
    pub length: u32,
    pub flags: u32,
    pub showCmd: u32,
    pub ptMinPosition: SerializablePOINT,
    pub ptMaxPosition: SerializablePOINT,
    pub rcNormalPosition: SerializableRECT,
}

#[derive(Serialize, Deserialize)]
pub struct SerializablePOINT {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableRECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

pub fn window_placement_to_serializable(wndpl: WINDOWPLACEMENT) -> SerializableWINDOWPLACEMENT {
    SerializableWINDOWPLACEMENT {
        length: wndpl.length,
        flags: wndpl.flags.0,
        showCmd: wndpl.showCmd.0,
        ptMinPosition: SerializablePOINT {
            x: wndpl.ptMinPosition.x,
            y: wndpl.ptMinPosition.y,
        },
        ptMaxPosition: SerializablePOINT {
            x: wndpl.ptMaxPosition.x,
            y: wndpl.ptMaxPosition.y,
        },
        rcNormalPosition: SerializableRECT {
            left: wndpl.rcNormalPosition.left,
            top: wndpl.rcNormalPosition.top,
            right: wndpl.rcNormalPosition.right,
            bottom: wndpl.rcNormalPosition.bottom,
        },
    }
}


fn serializable_to_window_placement(serializable_wndpl: SerializableWINDOWPLACEMENT) -> WINDOWPLACEMENT {
    WINDOWPLACEMENT {
        length: serializable_wndpl.length,
        flags: WINDOWPLACEMENT_FLAGS(serializable_wndpl.flags),
        showCmd: SHOW_WINDOW_CMD(serializable_wndpl.showCmd),
        ptMinPosition: POINT {
            x: serializable_wndpl.ptMinPosition.x,
            y: serializable_wndpl.ptMinPosition.y,
        },
        ptMaxPosition: POINT {
            x: serializable_wndpl.ptMaxPosition.x,
            y: serializable_wndpl.ptMaxPosition.y,
        },
        rcNormalPosition: RECT {
            left: serializable_wndpl.rcNormalPosition.left,
            top: serializable_wndpl.rcNormalPosition.top,
            right: serializable_wndpl.rcNormalPosition.right,
            bottom: serializable_wndpl.rcNormalPosition.bottom,
        },
    }
}

pub fn save_window_placement(hwnd: HWND, file_name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut wndpl = WINDOWPLACEMENT {
        length: std::mem::size_of::<WINDOWPLACEMENT>() as u32,
        flags: WINDOWPLACEMENT_FLAGS(0),
        showCmd: SHOW_WINDOW_CMD(0),
        ptMinPosition: POINT { x: 0, y: 0 },
        ptMaxPosition: POINT { x: 0, y: 0 },
        rcNormalPosition: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
    };

    unsafe {
        if GetWindowPlacement(hwnd, &mut wndpl).0 == 0 {
            return Err(Box::new(std::io::Error::last_os_error()));
        }
    }

    let serializable_wndpl = window_placement_to_serializable(wndpl);
    let serialized_wndpl = serde_json::to_string(&serializable_wndpl)?;
    std::fs::write(file_name, serialized_wndpl)?;

    Ok(())
}


pub fn load_window_placement(hwnd: HWND, file_name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string(file_name)?;
    let serializable_wndpl: SerializableWINDOWPLACEMENT = serde_json::from_str(&file_content)?;
    let wndpl = serializable_to_window_placement(serializable_wndpl);

    unsafe {
        if SetWindowPlacement(hwnd, &wndpl).0 == 0 {
            return Err(Box::new(std::io::Error::last_os_error()));
        }
    }

    Ok(())
}
