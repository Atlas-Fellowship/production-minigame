
export function sum(vals: number[]) {
  let result = 0;
  for (const val of vals) {
    result += val;
  }
  return result;
}

export function assert(cond: boolean, msg?: string): asserts cond {
  if (!cond) {
    throw new Error(msg);
  }
}
