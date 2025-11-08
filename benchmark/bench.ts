import { Bench } from 'tinybench'

import { plus100, getMainDiskInfo } from '../index.js'

function add(a: number) {
  return a + 100
}

const b = new Bench()

b.add('Native a + 100', () => {
  plus100(10)
})

b.add('JavaScript a + 100', () => {
  add(10)
})
b.add(' 计算c盘大小', () => {
  getMainDiskInfo()
})

await b.run()

console.table(b.table())
