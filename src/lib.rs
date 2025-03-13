mod fun;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
//use std::thread;
use std::ffi::{CString,CStr};
use std::os::raw::c_char;
use once_cell::sync::Lazy;
use std::thread::{self, JoinHandle};
pub struct SharedState {
    result: Option<String>,
    done: bool,
}

impl SharedState {
    // Add methods to get and set the result, and check if it's done
}

lazy_static::lazy_static! {
    static ref SHARED_STATES: Mutex<HashMap<usize, Arc<Mutex<SharedState>>>> = Mutex::new(HashMap::new());
}

#[no_mangle]
pub extern "C" fn create_shared_state() -> usize {
    let shared_state = Arc::new(Mutex::new(SharedState { result: None, done: false }));
    let mut shared_states = SHARED_STATES.lock().unwrap();
    let id = shared_state.as_ref() as *const _ as usize;
    shared_states.insert(id, shared_state);
    id
}
//static mut handle:Option<std::thread::JoinHandle<()>>=None;
// Wrap the Option<JoinHandle<()>> with Arc<Mutex<>> for thread-safe access
static HANDLE: Lazy<Arc<Mutex<Option<std::thread::JoinHandle<()>>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
#[no_mangle]
pub extern "C" fn create_window(shared_state_id: usize) {
    let shared_states = SHARED_STATES.lock().unwrap();
    let shared_state = shared_states.get(&shared_state_id).expect("Invalid shared state ID").clone();
	let handle_clone = Arc::clone(&HANDLE);
    let mut handle_guard = handle_clone.lock().unwrap();
    *handle_guard=Some(thread::spawn(move || {
        // Create and display the Win32API window here
		let a=fun::create_window();
        // When the window is done, update the shared state object with the result
        let mut shared_state = shared_state.lock().unwrap();
        shared_state.result = a;//Some("The result of the window operation".to_string());
        shared_state.done = true;
    }));
}

#[no_mangle]
pub extern "C" fn is_done(shared_state_id: usize) -> bool {
    let shared_states = SHARED_STATES.lock().unwrap();
    let shared_state = shared_states.get(&shared_state_id).expect("Invalid shared state ID").lock().unwrap();
    shared_state.done
}

#[no_mangle]
pub extern "C" fn get_result(shared_state_id: usize) -> *mut c_char {
    let shared_states = SHARED_STATES.lock().unwrap();
    let shared_state = shared_states.get(&shared_state_id).expect("Invalid shared state ID").lock().unwrap();
    let result = shared_state.result.as_ref().expect("Result not set").clone();
	//println!("result: {}",result);
    CString::new(result).expect("Could not convert").into_raw()
}

#[no_mangle]
pub extern "C" fn delete_shared_state(shared_state_id: usize) {
    let mut shared_states = SHARED_STATES.lock().unwrap();
    shared_states.remove(&shared_state_id);
}



/*fn main() {
	let i=create_shared_state();
    create_window(i);
	let handle_clone = Arc::clone(&HANDLE);

    // Lock the Mutex and try to join the thread if it exists
    let mut handle_guard = handle_clone.lock().unwrap();
    if let Some(handle) = handle_guard.take() {
        handle.join().unwrap();
		
		let c_str: &CStr = unsafe { CStr::from_ptr(get_result(i)) };
		// Convert &CStr to &str
    let rust_str: &str = c_str.to_str().expect("Failed to convert C string to Rust string");

    // Convert &str to String
    let rust_string: String = rust_str.to_owned();

    println!("Rust string: {}", rust_string);
    }
}*/