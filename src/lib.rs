#![deny(clippy::all)]
use napi_derive::napi;

// 导入Windows API相关模块
#[cfg(windows)]
use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
/// 将输入数字加上 100 并返回结果
///
/// # 参数
///
/// * `input` - 要加 100 的数字
///
/// # 返回值
///
/// 返回输入数字加 100 的结果
#[napi] // 使用 napi 宏标记此函数为 NAPI 可调用函数
pub fn plus_100(input: u32) -> u32 {
  // 定义一个名为 plus_100 的公共函数，接受一个 u32 类型的参数 input，返回一个 u32 类型的值
  input + 100 // 返回 input 加上 100 的结果
}

/// 磁盘信息结构体，包含总空间和可用空间信息
#[napi(object)]
pub struct DiskInfo {
  /// 总空间（字节）
  pub total_bytes: i64,
  /// 可用空间（字节）
  pub available_bytes: i64,
}

/// 获取 Windows 系统主磁盘（C 盘）的空间信息
///
/// # 返回值
///
/// 返回包含总空间和可用空间的 DiskInfo 结构体
#[napi]
#[cfg(windows)]
pub fn get_main_disk_info() -> DiskInfo {
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
      DiskInfo {
        total_bytes: total_bytes as i64,
        available_bytes: available_bytes as i64,
      }
    } else {
      DiskInfo {
        total_bytes: 0_i64,
        available_bytes: 0_i64,
      }
    }
  }
}

/// 获取非 Windows 系统（Unix/Linux/macOS）主磁盘（根目录）的空间信息
///
/// # 返回值
///
/// 返回包含总空间和可用空间的 DiskInfo 结构体
#[napi]
#[cfg(not(windows))]
pub fn get_main_disk_info() -> DiskInfo {
  use sysinfo::Disks; // 导入 sysinfo 库中的 Disks 结构体

  // 创建磁盘列表实例并刷新信息
  let disks = Disks::new_with_refreshed_list();

  // 预先确定目标挂载点，避免在循环中重复检查系统类型
  let target_mount_point =
    if cfg!(target_os = "linux") || cfg!(target_family = "unix") || cfg!(target_os = "macos") {
      Some("/") // Unix-like 系统的根目录挂载点
    } else {
      None // 其他系统暂时不支持
    };

  // 如果没有确定的目标挂载点，直接返回
  let target = match target_mount_point {
    Some(t) => t, // 使用确定的挂载点
    None => {
      return DiskInfo {
        // 返回空的磁盘信息
        total_bytes: 0_i64,
        available_bytes: 0_i64,
      };
    }
  };

  // 遍历所有磁盘，寻找主硬盘
  for disk in disks.list() {
    let mount_point = disk.mount_point().to_string_lossy(); // 获取磁盘挂载点

    if cfg!(target_os = "macos") {
      // macOS系统检查，需要额外检查是否包含"Macintosh"
      if mount_point == target || mount_point.contains("Macintosh") {
        return DiskInfo {
          // 返回找到的磁盘信息
          total_bytes: disk.total_space() as i64,
          available_bytes: disk.available_space() as i64,
        };
        // (disk.total_space(), disk.available_space());
      }
    } else {
      // Linux和其他Unix系统检查
      if mount_point == target {
        return PackageJson {
          // 注意：这里似乎有错误，应该返回 DiskInfo 而不是 PackageJson
          total_bytes: disk.total_space() as i64,
          available_bytes: disk.available_space() as i64,
        };
      }
    }
  }

  // 如果没有找到主硬盘，返回 (0, 0)
  DiskInfo {
    total_bytes: 0_i64,
    available_bytes: 0_i64,
  }
}
