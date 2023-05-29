use winapi::{shared::minwindef::HKL, um::winuser};

use crate::KeyboardIdentifier;

/// # Errors
/// 
/// Will return the `last_os_error` (`GetLastError`) if 0 keyboards are found or the function fails
pub fn list_keyboards() -> Result<Vec<KeyboardIdentifier>, std::io::Error> {
    unsafe {
        // this may be convertable into Option<i32>
        let maybe_keyboard_count = winuser::GetKeyboardLayoutList(0, std::ptr::null_mut());

        if maybe_keyboard_count == 0 {
            Err(std::io::Error::last_os_error())
        } else {
            let mut raw_keyboards: Vec<HKL> = vec![0 as HKL; maybe_keyboard_count as usize];

            let new_keyboard_count =
                winuser::GetKeyboardLayoutList(maybe_keyboard_count, raw_keyboards.as_mut_ptr());

            // resize in case a keyboard was removed since the first call
            // this should only ever shrink the array
            raw_keyboards.resize(new_keyboard_count as usize, std::ptr::null_mut());

            Ok(raw_keyboards.iter().map(|x| KeyboardIdentifier(*x as usize)).collect())
        }
    }
}
