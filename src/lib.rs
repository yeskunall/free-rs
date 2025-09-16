/// This is a 1-1 mapping of the vm_statistics64 struct, except that the fields (values) are all multiplied by the machine page size.
///
/// [vm_statistics64](https://github.com/apple-oss-distributions/xnu/blob/e3723e1f17661b24996789d8afc084c0c3303b26/osfmk/mach/vm_statistics.h#L137)
pub struct MemoryStats {
    pub free_count: u64,            // total number of free pages in the system.
    pub active_count: u64,          // total number of pages currently in use and pageable.
    pub inactive_count: u64,        // total number of pages on the inactive list.
    pub wire_count: u64, // total number of pages wired down.  That is, pages that cannot be paged out.
    pub zero_fill_count: u64, // total number of zero-filled pages.
    pub reactivations: u64, // total number of pages reactivated.
    pub pageins: u64, // total number of requests for pages from a pager (such as the inode pager).
    pub pageouts: u64, // total number of pages that have been paged out.
    pub faults: u64,  // number of times the "vm_fault" routine has been called.
    pub cow_faults: u64, // number of faults that caused a page to be copied (generally caused by copy-on-write faults).
    pub lookups: u64,    // total number of object cache lookups.
    pub hits: u64,       // total number of object cache hits.
    pub purges: u64,     // total number of pages purged.
    pub purgeable_count: u64, // total number of purgeable pages.
    pub speculative_count: u64, // total number of pages on the speculative list.
    pub decompressions: u64, // total number of pages that have been decompressed by the VM compressor.
    pub compressions: u64, // total number of pages that have been compressed by the VM compressor.
    pub swapins: u64, // total number of compressed pages that have been swapped back in (via compression segments) from disk.
    pub swapouts: u64, // total number of compressed pages that have been swapped out (via compression segments) to disk.
    pub compressor_page_count: u64, // total number of pages used by the compressed pager to hold all the compressed data.
    pub throttled_count: u64, // total number of pages on the throttled list (not wired but not pageable).
    pub external_page_count: u64, // total number of pages that are file-backed (non-swap).
    pub internal_page_count: u64, // total number of pages that are anonymous.
    pub total_uncompressed_pages_in_compressor: u64, // total number of pages that have been decompressed by the VM compressor.
}

pub fn get_libc_vm_stats() -> Result<libc::vm_statistics64, String> {
    unsafe {
        let mut count = libc::HOST_VM_INFO64_COUNT;
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

        Ok(stats)
    }
}

pub fn get_mem_stats() -> Result<MemoryStats, String> {
    let stats = get_libc_vm_stats().unwrap();

    unsafe {
        let page_size = libc::sysconf(libc::_SC_PAGESIZE) as u64;

        Ok(MemoryStats {
            free_count: stats.free_count as u64 * page_size,
            active_count: stats.active_count as u64 * page_size,
            inactive_count: stats.inactive_count as u64 * page_size,
            wire_count: stats.wire_count as u64 * page_size,
            zero_fill_count: stats.zero_fill_count * page_size,
            reactivations: stats.reactivations * page_size,
            pageins: stats.pageins * page_size,
            pageouts: stats.pageouts * page_size,
            faults: stats.faults * page_size,
            cow_faults: stats.cow_faults * page_size,
            lookups: stats.lookups * page_size,
            hits: stats.hits * page_size,
            purges: stats.purges * page_size,
            purgeable_count: stats.purgeable_count as u64 * page_size,
            speculative_count: stats.speculative_count as u64 * page_size,
            decompressions: stats.decompressions * page_size,
            compressions: stats.compressions * page_size,
            swapins: stats.swapins * page_size,
            swapouts: stats.swapouts * page_size,
            compressor_page_count: stats.compressor_page_count as u64 * page_size,
            throttled_count: stats.throttled_count as u64 * page_size,
            external_page_count: stats.external_page_count as u64 * page_size,
            internal_page_count: stats.internal_page_count as u64 * page_size,
            total_uncompressed_pages_in_compressor: stats.total_uncompressed_pages_in_compressor
                * page_size,
        })
    }
}
