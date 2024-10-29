export function resolveLocalUrl(path: string) {
  let baseUrl = import.meta.env.BASE_URL ?? "/";
  if (!baseUrl.endsWith("/")) {
    baseUrl += "/";
  }
  return new URL(`${baseUrl}${path}`, window.location.href);
}

export function chunk<T>(arr: T[], chunkSize: number): T[][] {
  const output: T[][] = [];
  let chunk: T[] = [];
  for (const x of arr) {
    chunk.push(x);
    if (chunk.length >= chunkSize) {
      output.push(chunk);
      chunk = [];
    }
  }
  if (chunk.length > 0) {
    output.push(chunk);
  }
  return output;
}
