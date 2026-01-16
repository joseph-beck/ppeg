const isNonEmptyArray = <T>(value: T[] | unknown): boolean => Array.isArray(value) && value.length > 0

export { isNonEmptyArray }
