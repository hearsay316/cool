## API 文档

### `getMainDiskInfo(): PackageJson`

获取C盘的空间信息，包括总空间和可用空间。

**返回值：**

- `PackageJson` 对象，包含以下属性：
  - `totalBytes` (number): C盘总空间（字节）
  - `availableBytes` (number): C盘可用空间（字节）

**示例：**

```javascript
const { getMainDiskInfo } = require('@bridge-rs/cool')
const diskInfo = getMainDiskInfo()
console.log(`总空间: ${diskInfo.totalBytes / 1024 / 1024 / 1024} GB`)
console.log(`可用空间: ${diskInfo.availableBytes / 1024 / 1024 / 1024} GB`)
```

## 使用示例

```javascript
const { getMainDiskInfo } = require('@bridge-rs/cool')

// 获取磁盘信息
const diskInfo = getMainDiskInfo()
console.log('磁盘信息:', diskInfo)
```
