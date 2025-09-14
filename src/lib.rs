use libc::sysctl;

/// Get usage statistics.
pub fn get_usage_statistics() -> Result<(u64, u64), Box<dyn std::error::Error>> {
    let usage: u64;
    let mut total: u64 = 0;

    unsafe {
        let mut mib = [libc::CTL_HW, libc::HW_MEMSIZE];
        let mut size = std::mem::size_of::<u64>();

        let kern_ret = sysctl(
            mib.as_mut_ptr(),
            mib.len() as _,
            &mut total as *mut _ as *mut _,
            &mut size,
            std::ptr::null_mut(),
            0,
        );

        if kern_ret != 0 {
            return Err("failed to get total memory".into());
        }
    }

    unsafe {
        let mut count = libc::HOST_VM_INFO64_COUNT;
        let page_size = libc::sysconf(libc::_SC_PAGESIZE) as u64;
        let mut stats = std::mem::zeroed::<libc::vm_statistics64>();

        let kern_ret = libc::host_statistics64(
            #[allow(deprecated)]
            libc::mach_host_self(),
            libc::HOST_VM_INFO64,
            &mut stats as *mut _ as *mut _,
            &mut count,
        );

        if kern_ret != 0 {
            return Err("failed to get memory stats".into());
        }

        usage = (stats.active_count as u64
            + stats.inactive_count as u64
            + stats.wire_count as u64
            + stats.speculative_count as u64
            + stats.compressor_page_count as u64
            - stats.purgeable_count as u64
            - stats.external_page_count as u64)
            * page_size;
    }

    Ok((total, usage))
}
