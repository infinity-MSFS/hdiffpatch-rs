use std::os::raw::{c_int, c_uchar, c_uint, c_void};

#[repr(C)]
pub struct HStreamInput {
    pub stream_begin: *const c_uchar,
    pub stream_end: *const c_uchar,
}

#[repr(C)]
pub struct HStreamOutput {
    pub stream_begin: *mut c_uchar,
    pub stream_end: *mut c_uchar,
}

extern "C" {
    pub fn patch(
        out_newData: *mut c_uchar,
        out_newData_end: *mut c_uchar,
        oldData: *const c_uchar,
        oldData_end: *const c_uchar,
        serializedDiff: *const c_uchar,
        serializedDiff_end: *const c_uchar,
    ) -> c_int;

}

pub fn apply_patch(out_new_data: &mut [u8], old_data: &[u8], serialized_diff: &[u8]) -> bool {
    let result = unsafe {
        patch(
            out_new_data.as_mut_ptr(),
            out_new_data.as_mut_ptr().add(out_new_data.len()),
            old_data.as_ptr(),
            old_data.as_ptr().add(old_data.len()),
            serialized_diff.as_ptr(),
            serialized_diff.as_ptr().add(serialized_diff.len()),
        )
    };

    result != 0
}
