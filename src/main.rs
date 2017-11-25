extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate widestring;


use std::ffi::CString;
//use std::os::windows::ffi::OsStrExt;
//use std::ffi::OsStr;
use widestring::WideCString;

// win api
use kernel32::LoadLibraryA;
use kernel32::LoadLibraryW;
use kernel32::GetProcAddress;

// types
use winapi::minwindef::{FARPROC, DWORD};

use winapi::winuser::{MB_OK, MB_ICONINFORMATION};

/* fn to_wstring(str: &str) -> Vec<u16> {
    let v: Vec<u16> =
        OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
    v
}
*/

fn main() {
    let msg_box_a: FARPROC;
    let msg_box_w: FARPROC;
    let msgbox_a_str = CString::new("MessageBoxA").unwrap();
    let msgbox_w_str = CString::new("MessageBoxW").unwrap();
    let user32_a_str = CString::new("user32.dll").unwrap();
    let user32_w_str = WideCString::from_str("user32.dll").unwrap();

    let msgbox_ansi: extern "stdcall" fn(winapi::HWND,
                                         winapi::LPCSTR,
                                         winapi::LPCSTR,
                                         winapi::UINT
    );

    let msgbox_wide: extern "stdcall" fn(winapi::HWND,
                                         winapi::LPCWSTR,
                                         winapi::LPCWSTR,
                                         winapi::UINT
    );

    let lp_text = CString::new("Hello, world!").unwrap();
    let lp_caption = CString::new("MessageBox Example ").unwrap();

    let lp_text_w = WideCString::from_str("Hello, world! 中华人民共和国").unwrap();
    let lp_caption_w = WideCString::from_str("MessageBox Example").unwrap();

    unsafe {
        msg_box_a = GetProcAddress(LoadLibraryA(user32_a_str.as_ptr()), msgbox_a_str.as_ptr());
        msg_box_w = GetProcAddress(LoadLibraryW(user32_w_str.as_ptr()), msgbox_w_str.as_ptr());
    }

    msgbox_ansi = unsafe { std::mem::transmute(msg_box_a) };
    msgbox_wide = unsafe { std::mem::transmute(msg_box_w) };


    println!("MessageBoxA Adress is {}", msg_box_a as DWORD);
    println!("MessageBoxW Adress is {}", msg_box_w as DWORD);

    msgbox_ansi(
        std::ptr::null_mut(),
        lp_text.as_ptr(),
        lp_caption.as_ptr(),
        MB_OK | MB_ICONINFORMATION
    );

    msgbox_wide(
        std::ptr::null_mut(),
        lp_text_w.as_ptr(),
        lp_caption_w.as_ptr(),
        MB_OK | MB_ICONINFORMATION
    );
}
