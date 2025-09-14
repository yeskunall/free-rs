use libc::{c_int, size_t, sysctl};

/// Returns a Mach system’s virtual memory page size in bytes.
///
/// Your architecture type determines your page size. For example, on Apple Silicon chips the default page size is 16KB.
pub fn get_page_size() -> usize {
    let mut page_size: size_t = 0;
    let mut size = std::mem::size_of::<size_t>();

    unsafe {
        let mib: [c_int; 2] = [libc::CTL_HW, libc::HW_PAGESIZE];
        sysctl(
            mib.as_ptr() as *mut c_int,
            2,
            &mut page_size as *mut _ as *mut _,
            &mut size,
            std::ptr::null_mut(),
            0,
        );
    }

    page_size
}

