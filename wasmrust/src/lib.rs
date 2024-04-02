use std::mem;
use std::slice;
use std::str;

#[no_mangle]
pub extern "C" fn alloc(len: i32) -> *const u8 {
    let mut buf = Vec::with_capacity((len) as usize);
    let ptr = buf.as_mut_ptr();
    // tell Rust not to clean this up
    mem::forget(buf);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: &mut u8, len: i32) {
    let _ = Vec::from_raw_parts(ptr, 0, len as usize);
}


#[no_mangle]
pub extern "C" fn greet(ptr: i32, len: i32) -> i32 {
    let bytes = unsafe { slice::from_raw_parts_mut(ptr as *mut u8, len as usize) };
    let str = "HelloGolou";
    let final_len = len + (str.len() as i32);
    let final_alloc = alloc(final_len + 1) as *mut u8;
    let result = unsafe { slice::from_raw_parts_mut(final_alloc, (final_len + 1) as usize) };

    let  a = (format!("{}{}", str, str::from_utf8(bytes).unwrap())).into_bytes();
  
    let mut idx: i32 = 1;
    result[0] = final_len as u8;
    for ch in a {
        result[idx as usize] = ch;
        idx += 1;
    }

    final_alloc as i32
}