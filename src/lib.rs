// Remove this
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::ptr;
use winapi::shared::minwindef::{
    HINSTANCE,
    DWORD,
    LPVOID,
    BOOL,
    TRUE,
    FALSE,
};
use winapi::um::winnt::{
    DLL_PROCESS_ATTACH,
    DLL_PROCESS_DETACH,
};
use winapi::um::libloaderapi::{
    DisableThreadLibraryCalls,
    FreeLibraryAndExitThread,
};
use winapi::um::processthreadsapi::{
    CreateThread,
    GetCurrentProcess,
};


unsafe extern "system" fn my_thread_function(lp_param: LPVOID) -> DWORD {
    /*



    */
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    lpv_reserved: LPVOID
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => { unsafe {
            DisableThreadLibraryCalls(hinst_dll);
            CreateThread(
                ptr::null_mut(),
                0,
                Some(my_thread_function),
                hinst_dll as LPVOID,
                0,
                ptr::null_mut(),
            );
        } }

        DLL_PROCESS_DETACH => {
            if !lpv_reserved.is_null() {

            }
        }

        _ => {}
    }

    TRUE
}
