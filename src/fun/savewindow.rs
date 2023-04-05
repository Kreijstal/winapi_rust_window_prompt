use serde::{Deserialize, Serialize};
use windows::Win32::UI::WindowsAndMessaging::WINDOWPLACEMENT;
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

