use std::mem;
pub use winapi::shared::windef::HMONITOR;
pub use winapi::um::winuser::{GetMonitorInfoW, MONITORINFOEXW};

#[no_mangle]
pub fn get_monitor_info(handle: i32) -> MONITORINFOEXW {
    unsafe {
        let cast = handle as winapi::shared::windef::HMONITOR;
        let mut monitor_info: MONITORINFOEXW = mem::zeroed();
        monitor_info.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
        let monitor_info_ptr = <*mut _>::cast(&mut monitor_info);
        GetMonitorInfoW(cast, monitor_info_ptr);
        monitor_info
    }
}
