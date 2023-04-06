# Rust Win32API Window Wrapper

This Rust library provides a simple and safe wrapper around the Win32API for creating and managing a window textbox. It allows you to create a window, and retrieve text from it.

## Features

- Thread-safe access to shared state objects
- Window creation and management using Win32API
- Exposes a C-compatible API for interop with other languages

## Prerequisites

- Rust compiler (latest stable version recommended)
- Windows platform

## Usage

1. Clone :

```bash
git clone git@github.com:Kreijstal/winapi_rust_window_prompt.git
```

2. Use the library in any code, in rust this would look like:

```rust
extern crate winapi_rust_window_prompt;

use winapi_rust_window_prompt::*;
```

3. Create a shared state object and use it to manage the window:

```rust
fn main() {
    // Create a shared state object
    let shared_state_id = create_shared_state();

    // Create a window using the shared state object
    create_window(shared_state_id);

    // Wait for the window to be done
    while !is_done(shared_state_id) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // Retrieve the result of the window operation
    let result = unsafe { CStr::from_ptr(get_result(shared_state_id)).to_str().unwrap() };
    println!("Result: {}", result);

    // Clean up the shared state object
    delete_shared_state(shared_state_id);
}
```

## API Reference

### `create_shared_state() -> usize`

Creates a new shared state object and returns its ID.

### `create_window(shared_state_id: usize)`

Creates a new window and associates it with the specified shared state object.

### `is_done(shared_state_id: usize) -> bool`

Returns `true` if the window associated with the specified shared state object is done, `false` otherwise.

### `get_result(shared_state_id: usize) -> *mut c_char`

Returns the result of the window operation associated with the specified shared state object as a null-terminated C string.

### `delete_shared_state(shared_state_id: usize)`

Deletes the specified shared state object.

## License

This project is licensed under the [MIT License](LICENSE).