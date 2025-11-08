# @bridge-rs/cool

一个高性能的磁盘信息获取工具，使用 Rust 编写，通过 NAPI-RS 为 Node.js 提供原生绑定。

## 特性

- 🚀 **高性能**：基于 Rust 实现，提供接近原生的性能
- 🔧 **跨平台支持**：支持 Windows、macOS 和 Linux
- 📦 **Node.js 兼容**：支持 Node.js 12 及以上版本
- 🛡️ **类型安全**：完整的 TypeScript 类型定义

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

- **最低要求**：Node.js 12.0.0
- **推荐版本**：Node.js 16.0.0 或更高

### 支持的操作系统

- ✅ **Windows** (x64, ia32, arm64)
- ✅ **macOS** (x64, arm64, universal)
- ✅ **Linux** (x64, arm64, arm)

## API

### `getMainDiskInfo(): PackageJson`

获取主磁盘（Windows 为 C 盘，Unix 系统为根目录）的空间信息。

**返回值：**

```typescript
interface PackageJson {
  totalBytes: number // 总空间（字节）
  availableBytes: number // 可用空间（字节）
}
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
import { getMainDiskInfo, PackageJson } from '@bridge-rs/cool'

const diskInfo: PackageJson = getMainDiskInfo()
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

欢迎提交 Issue 和 Pull Request！

## 更新日志

### v1.0.6

- 添加 Node.js 12 支持
- 改进跨平台兼容性
- 优化性能和内存使用
