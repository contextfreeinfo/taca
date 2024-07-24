export function fail(message?: string | null): never {
  throw Error(message ?? undefined);
}
