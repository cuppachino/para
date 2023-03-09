import { resolve as _resolve } from "path";

export function log(...args: any[]) {
  const br = () => console.log("\n---\n");
  br();
  console.log(...args);
  br();
}
