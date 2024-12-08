import { dataViewOf, getU32 } from "./util";

export interface AppExports {
  _initialize: (() => void) | undefined;
  _start: (() => void) | undefined;
  start: (() => void) | undefined;
  update: ((event: number) => void) | undefined;
}

export class Part {
  exports: AppExports = undefined as any;

  init(instance: WebAssembly.Instance) {
    this.exports = instance.exports as any;
    this.memory = instance.exports.memory as any;
  }

  memory: WebAssembly.Memory = undefined as any;

  #memoryBuffer: ArrayBuffer | null = null;
  #memoryBufferBytes: Uint8Array | null = null;
  #memoryBufferView: DataView | null = null;

  memoryBytes() {
    if (this.#memoryBuffer != this.memory.buffer) {
      // Either on first access or on internal reallocation.
      this.#memoryBuffer = this.memory.buffer;
      this.#memoryBufferBytes = new Uint8Array(this.#memoryBuffer);
      this.#memoryBufferView = new DataView(this.#memoryBuffer);
    }
    return this.#memoryBufferBytes!;
  }

  memoryView() {
    this.memoryBytes();
    return this.#memoryBufferView!;
  }

  memoryViewMake(ptr: number, len: number) {
    return new DataView(this.memory.buffer, ptr, len);
  }

  readAny<T>(
    spanPtr: number,
    itemSize: number,
    build: (view: DataView, offset: number) => T
  ): T[] {
    const view = dataViewOf(this.readBytes(spanPtr, itemSize));
    return [...Array(view.byteLength / itemSize).keys()].map((i) =>
      build(view, i * itemSize)
    );
  }

  readBytes(spanPtr: number, itemSize: number = 1) {
    // Can cache memory bytes when no app calls are being made.
    const memoryBytes = this.memoryBytes();
    const spanView = new DataView(memoryBytes.buffer, spanPtr, 2 * 4);
    // Wasm is explicitly little-endian.
    const contentPtr = getU32(spanView, 0);
    const contentLen = itemSize * getU32(spanView, 4);
    return memoryBytes.subarray(contentPtr, contentPtr + contentLen);
  }

  readString(spanPtr: number) {
    return textDecoder.decode(this.readBytes(spanPtr));
  }

  update(kind: number) {
    const { exports } = this;
    if (exports.update) {
      exports.update(kind);
    }
  }
}

export const textDecoder = new TextDecoder();
export const textEncoder = new TextEncoder();
