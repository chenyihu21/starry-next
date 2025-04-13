use axerrno::LinuxResult;
use axhal::time::{NANOS_PER_SEC, monotonic_time_nanos};
use starry_core::ctypes::SysInfo;
use arceos_posix_api::{
    self as api,
    ctypes::{_SC_PAGE_SIZE, _SC_NPROCESSORS_ONLN, _SC_PHYS_PAGES, _SC_AVPHYS_PAGES, _SC_OPEN_MAX},
};
use crate::ptr::{PtrWrapper, UserPtr};

pub fn sys_getuid() -> LinuxResult<isize> {
    Ok(0)
}

#[repr(C)]
pub struct UtsName {
    /// sysname
    pub sysname: [u8; 65],
    /// nodename
    pub nodename: [u8; 65],
    /// release
    pub release: [u8; 65],
    /// version
    pub version: [u8; 65],
    /// machine
    pub machine: [u8; 65],
    /// domainname
    pub domainname: [u8; 65],
}

impl Default for UtsName {
    fn default() -> Self {
        Self {
            sysname: Self::from_str("Starry"),
            nodename: Self::from_str("Starry - machine[0]"),
            release: Self::from_str("10.0.0"),
            version: Self::from_str("10.0.0"),
            machine: Self::from_str("10.0.0"),
            domainname: Self::from_str("https://github.com/BattiestStone4/Starry-On-ArceOS"),
        }
    }
}

impl UtsName {
    fn from_str(info: &str) -> [u8; 65] {
        let mut data: [u8; 65] = [0; 65];
        data[..info.len()].copy_from_slice(info.as_bytes());
        data
    }
}

pub fn sys_uname(name: UserPtr<UtsName>) -> LinuxResult<isize> {
    unsafe { *name.get()? = UtsName::default() };
    Ok(0)
}

/// get the system uptime and memory information.
/// # Arguments
/// * `info` - *mut SysInfo
pub fn sys_sysinfo(sysinfo: UserPtr<SysInfo>) -> LinuxResult<isize> {
    // let sysinfo = sysinfo.address().as_mut_ptr();
    // check if the pointer is valid
    // if sysinfo.is_null() {
    //     return Err(axerrno::LinuxError::EFAULT);
    // }
    // get the system uptime
    // let uptime = monotonic_time_nanos() / NANOS_PER_SEC;
    let page_size= api::sys_sysconf(_SC_PAGE_SIZE as _);
    let phys = api::sys_sysconf(_SC_PHYS_PAGES as _);
    let avail = api::sys_sysconf(_SC_AVPHYS_PAGES as _);
    let total_memory = page_size * phys;
    let free_memory = page_size * avail;
    let cpu_count = api::sys_sysconf(_SC_NPROCESSORS_ONLN as _);
    unsafe {
        *sysinfo.get()? = SysInfo {
            uptime: (monotonic_time_nanos() / NANOS_PER_SEC) as isize,
            loads: [0; 3],
            totalram: total_memory as usize,
            freeram: free_memory as usize,
            sharedram: 0,
            bufferram: 0,
            totalswap: 0,
            freeswap: 0,
            procs: cpu_count as u16,
            totalhigh: 0,
            freehigh: 0,
            mem_unit: 1,
        };
    }

    Ok(0)
}