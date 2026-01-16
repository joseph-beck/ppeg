import { CST } from '@/types/ppeg/cst'

const isLeaf = (node?: CST): boolean => node === undefined || node.children.length === 0

export { isLeaf }
