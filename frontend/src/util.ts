export function resolveLocalUrl(path: string) {
  let baseUrl = import.meta.env.BASE_URL ?? "/";
  if (!baseUrl.endsWith("/")) {
    baseUrl += "/";
  }
  return new URL(`${baseUrl}${path}`, window.location.href);
}
