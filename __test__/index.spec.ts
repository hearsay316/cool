import test from 'ava'

import { plus100, getMainDiskInfo, DiskInfo } from '../index'

test('sync function from native code', (t) => {
  const fixture = 42
  t.is(plus100(fixture), fixture + 100)
})

test('getMainDiskInfo should return correct disk info', (t) => {
  // 调用函数获取磁盘信息
  const diskInfo: DiskInfo = getMainDiskInfo()

  // 验证返回值的类型和结构
  t.true(typeof diskInfo === 'object')
  t.true('totalBytes' in diskInfo)
  t.true('availableBytes' in diskInfo)

  // 验证数值的合理性（这里假设磁盘大小不可能为负数）
  t.true(diskInfo.totalBytes >= 0)
  t.true(diskInfo.availableBytes >= 0)
  t.true(diskInfo.availableBytes <= diskInfo.totalBytes)

  // 打印实际结果（调试用）
  console.log('Disk Info:', diskInfo)
})
