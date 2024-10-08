export function dataViewOf(array: Uint8Array) {
  return new DataView(array.buffer, array.byteOffset, array.byteLength);
}

export function getU32(view: DataView, byteOffset: number) {
  return view.getUint32(byteOffset, true);
}

export function fail(message?: string | null): never {
  throw Error(message ?? undefined);
}

export function setF32(view: DataView, byteOffset: number, value: number) {
  return view.setFloat32(byteOffset, value, true);
}

export function setU8(view: DataView, byteOffset: number, value: number) {
  return view.setUint8(byteOffset, value);
}

export function setU16(view: DataView, byteOffset: number, value: number) {
  return view.setUint16(byteOffset, value, true);
}

export function setU32(view: DataView, byteOffset: number, value: number) {
  return view.setUint32(byteOffset, value, true);
}

export function setU64(view: DataView, byteOffset: number, value: bigint) {
  return view.setBigUint64(byteOffset, value, true);
}
