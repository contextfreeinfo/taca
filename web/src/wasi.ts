import { fail, getU32, setU16, setU32, setU64, setU8 } from "./util";

interface App {
  memoryViewMake(ptr: number, len: number): DataView;
  readString(spanPtr: number): string;
}

export function makeWasiEnv(app: App) {
  const outs = [] as string[];
  return {
    args_get() {
      return 0;
    },
    args_sizes_get(argvSize: number, argvBufSize: number) {
      setU32(app.memoryViewMake(argvSize, 4), 0, 0);
      setU32(app.memoryViewMake(argvBufSize, 4), 0, 0);
      return 0;
    },
    fd_close() {
      return 0;
    },
    fd_fdstat_get(fd: number, fdstat: number) {
      // Ignore fd for now. Just presume character device.
      const view = app.memoryViewMake(fdstat, 24);
      setU8(view, 0, 2); // 2 is for character device.
      setU8(view, 1, 0); // Just filler, but zero anyway.
      setU16(view, 2, 0); // Flags.
      setU32(view, 4, 0); // Also filler.
      setU64(view, 8, 0n); // Rights base.
      setU64(view, 16, 0n); // Rights inheriting.
      return 0;
    },
    fd_seek(fd: number, fileDelta: bigint, whence: number, newOffset: number) {
      setU64(app.memoryViewMake(newOffset, 8), 0, 0n);
      // Claim failure.
      // See https://github.com/ziglang/zig/blob/4d81e8ee915c3e012131cf90ed87cc8c6a01a934/stage1/wasi.c#L998
      return 29;
    },
    fd_write(fd: number, iovec: number, len: number, nwritten: number) {
      let total = 0;
      let text = outs[fd] ?? "";
      for (var i = 0; i < len; i += 1) {
        const offset = iovec + 8 * i;
        text += app.readString(offset);
        total += getU32(app.memoryViewMake(offset, 8), 4);
      }
      const lines = text.split("\n");
      for (var i = 0; i < lines.length - 1; i += 1) {
        if (fd == 1) {
          console.log(lines[i]);
        } else {
          // Not really an error, but use error for non-stdout for distinction.
          console.error(lines[i]);
        }
      }
      outs[fd] = lines.at(-1)!;
      return total;
    },
    proc_exit(code: number) {
      // We could also mark some exit state in the app, but meh.
      fail(`proc_exit(${code})`);
    },
  };
}
