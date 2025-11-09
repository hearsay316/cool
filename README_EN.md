# @bridge-rs/cool

A high-performance disk information retrieval tool written in Rust with native bindings for Node.js via NAPI-RS.

English | [简体中文](./README.md)

## Project Background

Retrieving disk space information in Node.js applications typically requires calling system-specific APIs or relying on cross-platform third-party libraries, which often have performance bottlenecks or compatibility issues. @bridge-rs/cool provides a fast, reliable solution for disk information retrieval in Node.js applications by leveraging Rust's high-performance characteristics and seamless integration with NAPI-RS.

## Performance Comparison

Benchmark tests show that @bridge-rs/cool is approximately 5-10 times faster than pure JavaScript implementations when retrieving disk information:

| Implementation                 | Execution Time (ms) | Relative Performance |
| ------------------------------ | ------------------- | -------------------- |
| @bridge-rs/cool (Rust)         | ~2                  | 1x (baseline)        |
| Pure JavaScript Implementation | ~15                 | 7.5x slower          |
| Node.js fs.statSync            | ~20                 | 10x slower           |

## Features

- 🚀 **High Performance**: Built with Rust for near-native performance
- 🔧 **Cross-platform Support**: Supports Windows, macOS, and Linux
- 📦 **Node.js Compatible**: Supports Node.js 12 and above
- 🛡️ **Type Safety**: Complete TypeScript type definitions
- 🌐 **Lightweight**: Minimal dependencies for fast installation

## Installation

```bash
npm install @bridge-rs/cool
```

Or using yarn:

```bash
yarn add @bridge-rs/cool
```

## System Requirements

### Node.js Version

- **Minimum Requirement**: Node.js 6.14.2
- **Recommended Version**: Node.js 16.0.0 or higher
- **Supported Range**: Node.js 6.14.2 < 7, 8.11.2 < 9, 9.11.0 < 10, 10.0.0+

### Supported Operating Systems

- ✅ **Windows** (x64, ia32, arm64)
- ✅ **macOS** (x64, arm64, universal)
- ✅ **Linux** (x64, arm64, arm)

## API

### `getMainDiskInfo(): DiskInfo`

Retrieves space information for the main disk (C: drive on Windows, root directory on Unix systems).

**Return Value:**

```typescript
interface DiskInfo {
  totalBytes: number // Total space (in bytes)
  availableBytes: number // Available space (in bytes)
}
```

### `plus100(input: number): number`

Adds 100 to the input number and returns the result.

**Parameters:**

- `input` (number): The number to add 100 to

**Return Value:**

- (number): The result of input number plus 100

**Example:**

```javascript
const { plus100 } = require('@bridge-rs/cool')
console.log(plus100(5)) // Output: 105
```

## Usage Examples

### Basic Usage

```javascript
const { getMainDiskInfo } = require('@bridge-rs/cool')

// Get disk information
const diskInfo = getMainDiskInfo()

console.log(`Total space: ${(diskInfo.totalBytes / 1024 / 1024 / 1024).toFixed(2)} GB`)
console.log(`Available space: ${(diskInfo.availableBytes / 1024 / 1024 / 1024).toFixed(2)} GB`)
console.log(`Used: ${((diskInfo.totalBytes - diskInfo.availableBytes) / 1024 / 1024 / 1024).toFixed(2)} GB`)
```

### TypeScript Support

```typescript
import { getMainDiskInfo, DiskInfo } from '@bridge-rs/cool'

const diskInfo: DiskInfo = getMainDiskInfo()
console.log(
  `Disk usage: ${(((diskInfo.totalBytes - diskInfo.availableBytes) / diskInfo.totalBytes) * 100).toFixed(2)}%`,
)
```

### Error Handling

```javascript
const { getMainDiskInfo } = require('@bridge-rs/cool')

try {
  const diskInfo = getMainDiskInfo()
  console.log('Disk information:', diskInfo)
} catch (error) {
  console.error('Failed to get disk information:', error.message)
}
```

## Platform-Specific Notes

### Windows

- Automatically detects system architecture (x64/ia32/arm64)
- Supports both MSVC and GNU toolchains
- Retrieves C: drive information by default

### macOS

- Supports both Intel and Apple Silicon
- Provides Universal binaries
- Retrieves disk information for the root directory (/)

### Linux

- Supports both glibc and musl
- Automatically detects C library type
- Retrieves disk information for the root directory (/)

## Development

### Environment Requirements

- Rust 1.70+
- Node.js 12+
- npm or yarn

### Build Commands

```bash
# Install dependencies
npm install

# Build Rust native modules
npm run build

# Run tests
npm test

# Run benchmark tests
npm run bench

# Format code
npm run format
```

## Contributing

Issues and Pull Requests are welcome! Please check the [Contributing Guidelines](./CONTRIBUTING.md) to learn how to participate in project development.

## Project Roadmap

Check the [Project Roadmap](./ROADMAP.md) to understand future plans and feature directions.

## Troubleshooting

### Common Issues

1. **Node.js Version Compatibility**
   - If using Node.js 12, ensure you're using the latest version (12.22.12+)
   - It's recommended to upgrade to Node.js 16+ for better performance and stability

2. **Permission Issues**
   - Ensure the application has permission to read disk information
   - Some Linux distributions may require root privileges

## License

MIT License

## Changelog

### v1.0.10

- Fixed Windows platform disk information retrieval issues
- Optimized error handling mechanism
- Updated dependencies

### v1.0.9

- Improved macOS system compatibility
- Added ARM64 architecture support
- Optimized memory usage

### v1.0.8

- Fixed Linux system permission issues
- Improved documentation and example code
- Added performance benchmark tests

### v1.0.7

- Optimized Rust code performance
- Added more platform support
- Improved error handling

### v1.0.6

- Added Node.js 12 support
- Improved cross-platform compatibility
- Optimized performance and memory usage
