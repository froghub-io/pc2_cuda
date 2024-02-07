use std::path::PathBuf;

extern "C" {
    fn treecr_c(
        cache_path: *const std::os::raw::c_char,
        replica_path: *const std::os::raw::c_char,
        sector_size: *const std::os::raw::c_char,
    ) -> usize;
}

pub fn treecr(
    cache_path: PathBuf,
    replica_path: PathBuf,
    sector_size: &str
) {

    let c_cache_path = std::ffi::CString::new(cache_path.to_str().unwrap()).unwrap();
    let c_replica_path = std::ffi::CString::new(replica_path.to_str().unwrap()).unwrap();
    let c_sector_size = std::ffi::CString::new(sector_size).unwrap();

    let _ = unsafe {
        treecr_c(
            c_cache_path.as_ptr(),
            c_replica_path.as_ptr(),
            c_sector_size.as_ptr()
        )
    };
}