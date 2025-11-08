#![deny(clippy::all)]
use napi_derive::napi;

// 导入Windows API相关模块
#[cfg(windows)]
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
#[cfg(windows)]
pub fn get_main_disk_info() -> PackageJson {
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
        total_bytes: 0_i64,
        available_bytes: 0_i64,
      }
    }
  }
}

/// 跨平台兼容的获取主硬盘信息函数
#[napi]
#[cfg(not(windows))]
pub fn get_main_disk_info() -> PackageJson {
  use sysinfo::Disks;

  // 创建磁盘列表实例并刷新信息
  let disks = Disks::new_with_refreshed_list();

  // 预先确定目标挂载点，避免在循环中重复检查系统类型
  let target_mount_point =
    if cfg!(target_os = "linux") || cfg!(target_family = "unix") || cfg!(target_os = "macos") {
      Some("/")
    } else {
      None
    };

  // 如果没有确定的目标挂载点，直接返回
  let target = match target_mount_point {
    Some(t) => t,
    None => {
      return PackageJson {
        total_bytes: 0_i64,
        available_bytes: 0_i64,
      }
    }
  };

  // 遍历所有磁盘，寻找主硬盘
  for disk in disks.list() {
    let mount_point = disk.mount_point().to_string_lossy();

    if cfg!(target_os = "macos") {
      // macOS系统检查，需要额外检查是否包含"Macintosh"
      if mount_point == target || mount_point.contains("Macintosh") {
        return PackageJson {
          total_bytes: disk.total_space() as i64,
          available_bytes: disk.available_space() as i64,
        };
        // (disk.total_space(), disk.available_space());
      }
    } else {
      // Linux和其他Unix系统检查
      if mount_point == target {
        return PackageJson {
          total_bytes: disk.total_space() as i64,
          available_bytes: disk.available_space() as i64,
        };
      }
    }
  }

  // 如果没有找到主硬盘，返回 (0, 0)
  PackageJson {
    total_bytes: 0_i64,
    available_bytes: 0_i64,
  }
}
