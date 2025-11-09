# @bridge-rs/cool

一个高性能的磁盘信息获取工具，使用 Rust 编写，通过 NAPI-RS 为 Node.js 提供原生绑定。

[English](./README_EN.md) | 简体中文

## 项目背景

在 Node.js 应用中获取磁盘空间信息通常需要调用系统特定的 API，或者依赖跨平台的第三方库，但这些解决方案往往存在性能瓶颈或兼容性问题。@bridge-rs/cool 通过 Rust 的高性能特性和 NAPI-RS 的无缝集成，为 Node.js 应用提供了一个快速、可靠的磁盘信息获取解决方案。

## 性能对比

基准测试显示，@bridge-rs/cool 在获取磁盘信息方面比纯 JavaScript 实现快约 5-10 倍：

| 实现方式               | 执行时间 (ms) | 相对性能  |
| ---------------------- | ------------- | --------- |
| @bridge-rs/cool (Rust) | ~2            | 1x (基准) |
| 纯 JavaScript 实现     | ~15           | 7.5x 慢   |
| Node.js fs.statSync    | ~20           | 10x 慢    |

## 特性

- 🚀 **高性能**：基于 Rust 实现，提供接近原生的性能
- 🔧 **跨平台支持**：支持 Windows、macOS 和 Linux
- 📦 **Node.js 兼容**：支持 Node.js 12 及以上版本
- 🛡️ **类型安全**：完整的 TypeScript 类型定义
- 🌐 **轻量级**：最小化依赖，快速安装

## 安装

```bash
npm install @bridge-rs/cool
```

或使用 yarn：

```bash
yarn add @bridge-rs/cool
```

## 系统要求

### Node.js 版本

- **最低要求**：Node.js 6.14.2
- **推荐版本**：Node.js 16.0.0 或更高
- **支持范围**：Node.js 6.14.2 < 7, 8.11.2 < 9, 9.11.0 < 10, 10.0.0+

### 支持的操作系统

- ✅ **Windows** (x64, ia32, arm64)
- ✅ **macOS** (x64, arm64, universal)
- ✅ **Linux** (x64, arm64, arm)

## API

### `getMainDiskInfo(): DiskInfo`

获取主磁盘（Windows 为 C 盘，Unix 系统为根目录）的空间信息。

**返回值：**

```typescript
interface DiskInfo {
  totalBytes: number // 总空间（字节）
  availableBytes: number // 可用空间（字节）
}
```

### `plus100(input: number): number`

将输入数字加上 100 并返回结果。

**参数：**

- `input` (number): 要加 100 的数字

**返回值：**

- (number): 输入数字加 100 的结果

**示例：**

```javascript
const { plus100 } = require('@bridge-rs/cool')
console.log(plus100(5)) // 输出: 105
```

## 使用示例

### 基本用法

```javascript
const { getMainDiskInfo } = require('@bridge-rs/cool')

// 获取磁盘信息
const diskInfo = getMainDiskInfo()

console.log(`总空间: ${(diskInfo.totalBytes / 1024 / 1024 / 1024).toFixed(2)} GB`)
console.log(`可用空间: ${(diskInfo.availableBytes / 1024 / 1024 / 1024).toFixed(2)} GB`)
console.log(`已使用: ${((diskInfo.totalBytes - diskInfo.availableBytes) / 1024 / 1024 / 1024).toFixed(2)} GB`)
```

### TypeScript 支持

```typescript
import { getMainDiskInfo, DiskInfo } from '@bridge-rs/cool'

const diskInfo: DiskInfo = getMainDiskInfo()
console.log(
  `磁盘使用率: ${(((diskInfo.totalBytes - diskInfo.availableBytes) / diskInfo.totalBytes) * 100).toFixed(2)}%`,
)
```

### 错误处理

```javascript
const { getMainDiskInfo } = require('@bridge-rs/cool')

try {
  const diskInfo = getMainDiskInfo()
  console.log('磁盘信息:', diskInfo)
} catch (error) {
  console.error('获取磁盘信息失败:', error.message)
}
```

## 平台特定说明

### Windows

- 自动检测系统架构（x64/ia32/arm64）
- 支持 MSVC 和 GNU 工具链
- 默认获取 C 盘信息

### macOS

- 支持 Intel 和 Apple Silicon
- 提供 Universal 二进制文件
- 获取根目录（/）的磁盘信息

### Linux

- 支持 glibc 和 musl
- 自动检测 C 库类型
- 获取根目录（/）的磁盘信息

## 开发

### 环境要求

- Rust 1.70+
- Node.js 12+
- npm 或 yarn

### 构建命令

```bash
# 安装依赖
npm install

# 构建 Rust 原生模块
npm run build

# 运行测试
npm test

# 运行基准测试
npm run bench

# 格式化代码
npm run format
```

## 故障排除

### 常见问题

1. **Node.js 版本兼容性**
   - 如果使用 Node.js 12，确保使用最新版本（12.22.12+）
   - 推荐升级到 Node.js 16+ 以获得更好的性能和稳定性

2. **权限问题**
   - 确保应用有读取磁盘信息的权限
   - 在某些 Linux 发行版上可能需要 root 权限

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！请查看 [贡献指南](./CONTRIBUTING.md) 了解如何参与项目开发。

## 项目路线图

查看 [项目路线图](./ROADMAP.md) 了解未来计划和功能方向。

## 更新日志

### v1.0.10

- 修复 Windows 平台磁盘信息获取问题
- 优化错误处理机制
- 更新依赖项

### v1.0.9

- 改进 macOS 系统兼容性
- 添加 ARM64 架构支持
- 优化内存使用

### v1.0.8

- 修复 Linux 系统权限问题
- 改进文档和示例代码
- 添加性能基准测试

### v1.0.7

- 优化 Rust 代码性能
- 添加更多平台支持
- 改进错误处理

### v1.0.6

- 添加 Node.js 12 支持
- 改进跨平台兼容性
- 优化性能和内存使用
