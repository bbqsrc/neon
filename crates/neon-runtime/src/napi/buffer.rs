use raw::{Env, Local};
use std::os::raw::c_void;

use nodejs_sys as napi;

pub unsafe extern "C" fn new(_out: &mut Local, _size: u32) -> bool { unimplemented!() }

pub unsafe extern "C" fn uninitialized(_out: &mut Local, _size: u32) -> bool { unimplemented!() }

pub unsafe extern "C" fn data<'a, 'b>(env: Env, base_out: &'a mut *mut c_void, obj: Local) -> usize {
    let mut size = 0;
    assert_eq!(
        napi::napi_get_buffer_info(env, obj, base_out as *mut _, &mut size as *mut _),
        napi::napi_status::napi_ok,
    );
    size
}
