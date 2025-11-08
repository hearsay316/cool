#![deny(clippy::all)]
use napi_derive::napi;
use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}
#[napi(object)]
pub struct PackageJson {
  pub total_bytes: i64,
  pub available_bytes: i64,
}

#[napi]
pub fn get_main_disk_info() -> PackageJson  {
  use std::ffi::OsStr;
  use std::os::windows::ffi::OsStrExt;

  // C: 盘符的宽字符表示
  let c_drive: Vec<u16> = OsStr::new("C:\\").encode_wide().chain(Some(0)).collect();

  let mut free_bytes: u64 = 0;
  let mut total_bytes: u64 = 0;
  let mut available_bytes: u64 = 0;

  unsafe {
    // 直接调用Windows API获取磁盘空间信息
    let result = GetDiskFreeSpaceExW(
      windows::core::PCWSTR(c_drive.as_ptr()),
      Some(&mut free_bytes as *mut u64),
      Some(&mut total_bytes as *mut u64),
      Some(&mut available_bytes as *mut u64),
    );

    if result.is_ok() {
      PackageJson {
        total_bytes: total_bytes as i64,
        available_bytes: available_bytes as i64,
      }
    } else {
      PackageJson {
        total_bytes: total_bytes as i64,
        available_bytes: available_bytes as i64,
      }
    }
  }
}