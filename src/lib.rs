use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct MemMetrics {
    pub ram_total: u64,  // bytes
    pub ram_usage: u64,  // bytes
    pub swap_total: u64, // bytes
    pub swap_usage: u64, // bytes
}

pub type WithError<T> = Result<T, Box<dyn std::error::Error>>;

pub fn get_libc_ram() -> WithError<(u64, u64)> {
    let (mut total, mut _usage) = (0u64, 0u64);

    unsafe {
        let mut name = [libc::CTL_HW, libc::HW_MEMSIZE];
        let mut size = std::mem::size_of::<u64>();
        let ret_code = libc::sysctl(
            name.as_mut_ptr(),
            name.len() as _,
            &mut total as *mut _ as *mut _,
            &mut size,
            std::ptr::null_mut(),
            0,
        );

        if ret_code != 0 {
            return Err("Failed to get total memory".into());
        }
    }

    unsafe {
        let mut count: u32 = libc::HOST_VM_INFO64_COUNT as _;
        let mut stats = std::mem::zeroed::<libc::vm_statistics64>();

        // todo: https://github.com/JohnTitor/mach2/issues/34
        #[allow(deprecated)]
        let ret_code = libc::host_statistics64(
            libc::mach_host_self(),
            libc::HOST_VM_INFO64,
            &mut stats as *mut _ as *mut _,
            &mut count,
        );

        if ret_code != 0 {
            return Err("Failed to get memory stats".into());
        }

        let page_size_kb = libc::sysconf(libc::_SC_PAGESIZE) as u64;

        _usage = (stats.active_count as u64
            + stats.inactive_count as u64
            + stats.wire_count as u64
            + stats.speculative_count as u64
            + stats.compressor_page_count as u64
            - stats.purgeable_count as u64
            - stats.external_page_count as u64)
            * page_size_kb;
    }

    Ok((total, _usage))
}

pub fn get_libc_swap() -> WithError<(u64, u64)> {
    let (mut total, mut _usage) = (0u64, 0u64);

    unsafe {
        let mut name = [libc::CTL_HW, libc::VM_SWAPUSAGE];
        let mut size = std::mem::size_of::<libc::xsw_usage>();
        let xsw = std::mem::zeroed::<libc::xsw_usage>();

        let kern_ret = libc::sysctl(
            name.as_mut_ptr(),
            name.len() as _,
            &mut total as *mut _ as *mut _,
            &mut size,
            std::ptr::null_mut(),
            0,
        );

        if kern_ret != 0 {
            return Err("failed to get total swap".into());
        }

        total = xsw.xsu_total;
        _usage = xsw.xsu_used;
    }

    Ok((total, _usage))
}

pub fn get_mem_usage() -> WithError<MemMetrics> {
    let (ram_total, ram_usage) = get_libc_ram()?;
    let (swap_total, swap_usage) = get_libc_swap()?;

    Ok(MemMetrics {
        ram_total,
        ram_usage,
        swap_total,
        swap_usage,
    })
}
