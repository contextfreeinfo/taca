import {
  default as cana,
  lz4Decompress,
  type Shader,
  shaderNew,
  shaderToGlsl,
  ShaderStage,
} from "../pkg/cana";
import { TexturePipeline, shaderProgramBuild } from "./drawing";
import { fail } from "./util";

export interface AppConfig {
  canvas: HTMLCanvasElement;
  code?: ArrayBuffer | Promise<Response>;
  runtimeWasm?: Promise<Response>;
}

export async function runApp(config: AppConfig) {
  const [appData] = await Promise.all([
    loadAppData(config.code!),
    cana(config.runtimeWasm),
  ]);
  if (appData) {
    await loadApp({ ...config, code: appData });
  }
}

/** Allows for additional custom properties. */
class App {
  constructor(config: AppConfig) {
    const canvas = (this.canvas = config.canvas);
    canvas.addEventListener("mousemove", (event) => {
      const rect = canvas.getBoundingClientRect();
      this.pointerPos = [event.clientX - rect.left, event.clientY - rect.top];
    });
    canvas.addEventListener("touchmove", (event) => {
      event.preventDefault();
      const touch = event.touches[0];
      const rect = canvas.getBoundingClientRect();
      this.pointerPos = [touch.clientX - rect.left, touch.clientY - rect.top];
    });
    this.config = config;
    this.gl = config.canvas.getContext("webgl2")!;
    this.texturePipeline = new TexturePipeline(this.gl);
    // Resize will fail if we couldn't get a context.
    this.resizeCanvas();
    // TODO Track for deregistration needs?
    new ResizeObserver(() => (this.resizeNeeded = true)).observe(config.canvas);
  }

  #attributesBuild(program: WebGLProgram, pipelineInfo: PipelineInfo) {
    const vertexAttrs: AttrInfo[] = [];
    const vertexBuffers: BufferInfo[] = [];
    const preBuffers = pipelineInfo.vertexBuffers;
    const { gl } = this;
    const attribCount = gl.getProgramParameter(program, gl.ACTIVE_ATTRIBUTES);
    let offset = 0;
    let bufferIndex = 0;
    const initBufferInfo = (firstAttr: number): BufferInfo => {
      const result = {
        firstAttr,
        step: 0,
        stride: 0,
        ...((pipelineInfo.vertexBuffers[bufferIndex] ?? {}) as BufferInfo | {}),
      };
      vertexBuffers.push(result);
      return result;
    };
    let bufferInfo = initBufferInfo(0);
    for (let i = 0; i < attribCount; i += 1) {
      buffers: while (bufferIndex + 1 < preBuffers.length) {
        if (i >= preBuffers[bufferIndex + 1].firstAttr) {
          bufferInfo.stride = offset;
          bufferIndex += 1;
          bufferInfo = initBufferInfo(i);
          offset = 0;
        } else {
          break buffers;
        }
      }
      const info = gl.getActiveAttrib(program, i) ?? fail();
      const loc = gl.getAttribLocation(program, info.name);
      const [size, type] =
        {
          [gl.FLOAT]: [1, gl.FLOAT],
          [gl.FLOAT_VEC2]: [2, gl.FLOAT],
          [gl.FLOAT_VEC4]: [4, gl.FLOAT],
        }[info.type] ?? fail();
      const typeSize = { [gl.FLOAT]: 4 }[type] ?? fail();
      // Pad for alignment.
      offset = Math.ceil(offset / typeSize) * typeSize;
      vertexAttrs.push({
        count: info.size,
        loc,
        offset,
        size,
        type,
      });
      offset += size * typeSize;
    }
    bufferInfo.stride = offset;
    const result: PipelineInfo = {
      ...pipelineInfo,
      vertexAttrs,
      vertexBuffers,
    };
    return result;
  }

  bindingApply(bindingsPtr: number) {
    const { buffers } = this;
    // Minimize allocations because this is in the draw loop.
    const view = this.memoryView();
    const vertexPtr = getU32(view, bindingsPtr);
    const vertexLen = getU32(view, bindingsPtr + 4);
    const index = buffers[getU32(view, bindingsPtr + 8) - 1];
    // TODO Predefine bindings to avoid allocations?
    const vertex = new Array<Buffer>(vertexLen);
    for (var i = 0; i < vertexLen; i += 1) {
      vertex[i] = buffers[getU32(view, vertexPtr + 4 * i) - 1];
    }
    this.binding = { index, vertex };
  }

  binding: Binding | null = null;
  bindingDefault: Binding | null = null;

  bufferNew(type: number, info: number) {
    const infoBytes = this.memoryViewMake(info, 2 * 4);
    const ptr = getU32(infoBytes, 0);
    const size = getU32(infoBytes, 4);
    // TODO Null ptr -> zero buffer for writing.
    const data = this.memoryBytes().subarray(ptr, ptr + size);
    const { gl } = this;
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    const buffer = gl.createBuffer();
    buffer || fail();
    this.buffers.push({
      buffer: buffer!,
      kind: ["vertex", "index"][type] as "vertex" | "index",
    });
    const target = [gl.ARRAY_BUFFER, gl.ELEMENT_ARRAY_BUFFER][type] ?? fail();
    gl.bindBuffer(target, buffer);
    const usageValue = ptr ? gl.STATIC_DRAW : gl.STREAM_DRAW;
    gl.bufferData(target, data, usageValue);
    return this.buffers.length;
  }

  buffered = false;

  #bufferedEnsure() {
    if (!this.buffered) {
      this.#pipelinedEnsure();
      if (!this.binding) {
        let { bindingDefault } = this;
        if (!this.bindingDefault) {
          const { buffers } = this;
          const find = (kind: string) =>
            buffers.find((buffer) => buffer.kind == kind) ?? fail();
          this.bindingDefault = bindingDefault = {
            index: find("index"),
            vertex: [find("vertex")],
          };
        }
        this.binding = bindingDefault;
      }
      this.#buffersBind();
    }
  }

  buffers: Buffer[] = [];

  #buffersBind() {
    // If at least two buffers, presumes one is data and one index.
    const { binding, gl, pipeline } = this;
    // Vertex buffer.
    const vertex = binding!.vertex[0];
    gl.bindBuffer(gl.ARRAY_BUFFER, vertex.buffer);
    const { stride } = pipeline!.buffers[0];
    for (const attr of pipeline!.attributes) {
      const { loc, offset, size, type } = attr;
      gl.enableVertexAttribArray(loc);
      gl.vertexAttribPointer(loc, size, type, false, stride, offset);
    }
    // TODO Instance buffer.
    // Index buffer.
    const index = binding!.index;
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, index.buffer);
    this.buffered = true;
  }

  canvas: HTMLCanvasElement;

  config: AppConfig;

  draw(itemBegin: number, itemCount: number, instanceCount: number) {
    // console.log(`draw(${itemBegin}, ${itemCount}, ${instanceCount})`);
    this.#bufferedEnsure();
    const { gl } = this;
    gl.drawElements(gl.TRIANGLES, itemCount, gl.UNSIGNED_SHORT, itemBegin);
  }

  drawText(text: string, x: number, y: number) {
    if (!text) return;
    if (text != this.textTextureText) {
      // TODO Consider font, color, and so on.
      // TODO LRU cache on atlas as separate helper library?
      this.textTexture = this.textDraw(text, this.textTexture || undefined);
      this.textTextureText = text;
    }
    this.drawTexture(this.textTexture, x, y);
  }

  drawTexture(textureIndex: number, x: number, y: number) {
    const {
      canvas: { clientWidth, clientHeight },
      gl,
      pipeline,
      textures,
    } = this;
    const { size, texture, usedSize } = textures[textureIndex - 1];
    this.texturePipeline.draw(
      texture,
      clientWidth,
      clientHeight,
      x,
      y,
      size,
      usedSize
    );
    if (pipeline) {
      gl.useProgram(pipeline.program);
    }
  }

  exports: AppExports = undefined as any;

  frameCommit() {
    this.buffered = this.passBegun = false;
    this.pipeline = null;
  }

  frameCount: number = 0;

  frameEnd() {
    if (this.passBegun) {
      this.frameCommit();
    }
    const frameWrap = 1000;
    this.frameCount += 1;
    this.frameCount = this.frameCount % frameWrap;
    if (!this.frameCount) {
      // TODO Instead do exponential decay estimate discarding outliers?
      // TODO Debugger or changing tabs can pause things.
      const now = Date.now();
      const elapsed = (now - this.frameTimeBegin) * 1e-3;
      const fps = frameWrap / elapsed;
      console.log(`fps: ${fps}`);
      this.frameTimeBegin = now;
    }
  }

  frameTimeBegin: number = Date.now();

  gl: WebGL2RenderingContext;

  indexBuffer: Buffer | null = null;

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

  offscreen = new OffscreenCanvas(1, 1);
  offscreenContext = this.offscreen.getContext("2d") ?? fail();

  passBegin() {
    let { gl, resizeNeeded } = this;
    if (resizeNeeded) this.resizeCanvas();
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    this.passBegun = true;
  }

  passBegun = false;

  pipeline: Pipeline | null = null;

  pipelineApply(pipelinePtr: number) {
    let { gl, pipelines } = this;
    const pipeline = (this.pipeline = pipelines[pipelinePtr - 1] ?? fail());
    gl.useProgram(pipeline.program);
    // Presume we need new buffer binding when the program changes.
    this.buffered = false;
  }

  #pipelineBuild(pipelineInfo: PipelineInfo) {
    const { gl, pipelines, shaders } = this;
    const shaderMake = (info: ShaderInfo, stage: ShaderStage) =>
      shaderToGlsl(shaders[info.shader - 1], stage, info.entry);
    const vertex = shaderMake(pipelineInfo.vertex, ShaderStage.Vertex);
    const fragment = shaderMake(pipelineInfo.fragment, ShaderStage.Fragment);
    // console.log(vertex);
    // console.log(fragment);
    const program = shaderProgramBuild(gl, vertex, fragment);
    pipelineInfo = this.#attributesBuild(program, pipelineInfo);
    const uniforms = this.#uniformsBuild(program);
    pipelines.push({
      attributes: pipelineInfo.vertexAttrs,
      buffers: pipelineInfo.vertexBuffers,
      program,
      uniforms,
    });
    return pipelineInfo;
  }

  #pipelineEnsure() {
    if (!this.pipelines.length) {
      this.#pipelineBuild(pipelineInfoDefault());
    }
  }

  #pipelinedEnsure() {
    if (!this.pipeline) {
      this.#pipelineEnsure();
      if (!this.passBegun) this.passBegin();
      if (this.pipelines.length > 0) this.pipelineApply(1);
    }
  }

  pipelineNew(info: number) {
    let pipelineInfo = this.pipelineInfoRead(info);
    console.log(pipelineInfo);
    pipelineInfo = this.#pipelineBuild(pipelineInfo);
    console.log(pipelineInfo);
    // console.log(this.pipelines);
    return this.pipelines.length;
  }

  private pipelineInfoRead(info: number): PipelineInfo {
    // TODO Can wit-bindgen or flatbuffers automate some of this?
    const infoView = this.memoryViewMake(info, 10 * 4);
    const readShaderInfo = (offset: number) => {
      return {
        entry: this.readString(infoView.byteOffset + offset),
        shader: getU32(infoView, offset + 2 * 4),
      };
    };
    const pipelineInfo: PipelineInfo = {
      fragment: readShaderInfo(0 * 4),
      vertex: readShaderInfo(3 * 4),
      vertexAttrs: this.readAny(
        info + 6 * 4,
        2 * 4,
        (view, offset): AttrInfo => ({
          count: 1,
          loc: getU32(view, offset),
          offset: getU32(view, offset + 1 * 4),
          size: 0,
          type: 0,
        })
      ),
      vertexBuffers: this.readAny(
        info + 8 * 4,
        3 * 4,
        (view, offset): BufferInfo => ({
          firstAttr: getU32(view, offset),
          step: getU32(view, offset + 1 * 4),
          stride: getU32(view, offset + 2 * 4),
        })
      ),
    };
    return pipelineInfoDefault(pipelineInfo);
  }

  pipelines: Pipeline[] = [];

  pointerPos: [x: number, y: number] = [0, 0];

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

  resizeCanvas() {
    const { canvas } = this.config;
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;
    this.gl.viewport(0, 0, canvas.width, canvas.height);
    this.resizeNeeded = false;
    this.tacaBufferUpdate();
  }

  resizeNeeded = false;

  shaders: Shader[] = [];
  tacaBuffer: WebGLBuffer | null = null;

  tacaBufferUpdate() {
    const { canvas } = this.config;
    if (this.tacaBuffer) {
      const { gl } = this;
      for (const pipeline of this.pipelines) {
        gl.bindBuffer(gl.UNIFORM_BUFFER, this.tacaBuffer);
        const tacaBytes = new Uint8Array(pipeline.uniforms.tacaSize);
        const tacaView = new DataView(tacaBytes.buffer);
        tacaView.setFloat32(0, canvas.width, true);
        tacaView.setFloat32(4, canvas.height, true);
        // console.log(tacaBytes);
        gl.bufferSubData(gl.UNIFORM_BUFFER, 0, tacaBytes);
      }
    }
  }

  textDraw(text: string, textureIndex?: number) {
    const { gl, offscreen, offscreenContext, textures } = this;
    const font = "30px sans-serif";
    offscreenContext.font = font;
    const metrics = offscreenContext.measureText(text);
    // console.log(metrics);
    const width = metrics.width;
    const height =
      metrics.fontBoundingBoxAscent + metrics.fontBoundingBoxDescent;
    // TODO Instead allow larger and use subtexture or even texture atlas?
    if (offscreen.width < width) offscreen.width = Math.ceil(width);
    if (offscreen.height < height) offscreen.height = Math.ceil(height);
    // TODO Clear only portion and do sub texImage2D thing?
    offscreenContext.clearRect(0, 0, offscreen.width, offscreen.height);
    offscreenContext.fillStyle = "white";
    offscreenContext.font = font;
    offscreenContext.textBaseline = "bottom";
    offscreenContext.fillText(text, 0, height);
    // console.log(data.data.reduce((x, y) => x + Math.sign(y)) / data.data.length);
    let makeNew = !textureIndex;
    let texture: WebGLTexture;
    if (textureIndex) {
      const textureInfo = textures[textureIndex - 1];
      if (
        textureInfo.size[0] < offscreen.width ||
        textureInfo.size[1] < offscreen.height
      ) {
        gl.deleteTexture(textureInfo.texture);
        makeNew = true;
      } else {
        texture = textureInfo.texture;
        textureInfo.usedSize = [width, height];
      }
    }
    if (makeNew) {
      texture = gl.createTexture() ?? fail();
      // TODO Simple {x, y} type for these things?
      const textureInfo: Texture = {
        size: [offscreen.width, offscreen.height],
        texture: texture,
        usedSize: [width, height],
      };
      if (!textureIndex) {
        textureIndex = textures.length + 1;
      }
      textures[textureIndex - 1] = textureInfo;
    }
    gl.bindTexture(gl.TEXTURE_2D, texture!);
    if (makeNew) {
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    }
    // The hope is that using a canvas as the source stays on gpu.
    gl.texImage2D(
      gl.TEXTURE_2D,
      0,
      gl.SRGB8_ALPHA8, // <- TODO Makes a difference at all?
      gl.RGBA,
      gl.UNSIGNED_BYTE,
      offscreen
    );
    return textureIndex!;
  }

  textTexture: number = 0;
  textTextureText: string = "";
  texturePipeline: TexturePipeline;
  textures: Texture[] = [];

  uniformsApply(uniforms: number) {
    this.#pipelinedEnsure();
    const { gl } = this;
    if (!this.uniformsBuffer) {
      const { pipeline } = this;
      const uniformsBuffer = gl.createBuffer() ?? fail();
      const { uniforms } = pipeline!;
      gl.bindBuffer(gl.UNIFORM_BUFFER, uniformsBuffer);
      gl.bufferData(gl.UNIFORM_BUFFER, uniforms.size, gl.STREAM_DRAW);
      for (let i = 0; i < uniforms.count; i += 1) {
        if (i != uniforms.tacaIndex) {
          gl.bindBufferBase(gl.UNIFORM_BUFFER, i + 1, uniformsBuffer);
        }
      }
      this.uniformsBuffer = uniformsBuffer;
      // Custom taca uniforms.
      const tacaBuffer = gl.createBuffer() ?? fail();
      gl.bindBuffer(gl.UNIFORM_BUFFER, tacaBuffer);
      gl.bufferData(gl.UNIFORM_BUFFER, uniforms.tacaSize, gl.STREAM_DRAW);
      gl.bindBufferBase(gl.UNIFORM_BUFFER, uniforms.tacaIndex + 1, tacaBuffer);
      this.tacaBuffer = tacaBuffer;
      this.tacaBufferUpdate();
    }
    const uniformsBytes = this.readBytes(uniforms);
    gl.bindBuffer(gl.UNIFORM_BUFFER, this.uniformsBuffer);
    gl.bufferSubData(gl.UNIFORM_BUFFER, 0, uniformsBytes);
  }

  uniformsBuffer: WebGLBuffer | null = null;

  #uniformsBuild(program: WebGLProgram): Uniforms {
    const { gl } = this;
    const count = gl.getProgramParameter(program, gl.ACTIVE_UNIFORM_BLOCKS);
    let size = -1;
    let tacaIndex = 0;
    let tacaSize = 0;
    // console.log(`uniforms: ${count}`);
    for (let i = 0; i < count; i += 1) {
      const name = gl.getActiveUniformBlockName(program, i);
      // console.log(`uniform: ${name}`);
      const nextSize =
        gl.getActiveUniformBlockParameter(
          program,
          i,
          gl.UNIFORM_BLOCK_DATA_SIZE
        ) ?? fail();
      if (name == "taca_uniform_block") {
        tacaIndex = i;
        tacaSize = nextSize;
      } else {
        if (size > 0 && nextSize != size) fail();
        size = nextSize;
      }
      gl.uniformBlockBinding(program, i, i + 1);
    }
    return { count, size, tacaIndex, tacaSize };
  }

  vertexBuffer: Buffer | null = null;
}

interface AppExports {
  listen: ((event: number) => void) | undefined;
  _start: () => void;
}

interface AttrInfo {
  count: number;
  loc: number;
  offset: number;
  size: number;
  type: number;
}

interface Binding {
  // TODO images/textures
  index: Buffer;
  vertex: Buffer[];
}

interface Buffer {
  buffer: WebGLBuffer;
  kind: "index" | "vertex";
}

interface BufferInfo {
  firstAttr: number;
  step: number;
  stride: number;
}

function dataViewOf(array: Uint8Array) {
  return new DataView(array.buffer, array.byteOffset, array.byteLength);
}

function getU32(view: DataView, byteOffset: number) {
  return view.getUint32(byteOffset, true);
}

async function loadApp(config: AppConfig) {
  const appData = config.code as ArrayBuffer;
  config.code = undefined;
  const appBytes = new Uint8Array(appData);
  const actualData =
    appBytes[0] == 4
      ? // Presume lz4 because wasm starts with 0.
        lz4Decompress(appBytes).buffer
      : appData;
  let app = new App(config);
  const env = makeAppEnv(app);
  let { instance } = await WebAssembly.instantiate(actualData, { env });
  app.init(instance);
  // TODO Fold config into start once we get fully off miniquad.
  const exports = app.exports;
  exports._start();
  if (exports.listen) {
    const update = () => {
      try {
        exports.listen!(0);
      } finally {
        app.frameEnd();
      }
      requestAnimationFrame(update);
    };
    requestAnimationFrame(update);
  }
}

async function loadAppData(code: ArrayBuffer | Promise<Response>) {
  if (code instanceof ArrayBuffer) {
    return code;
  } else {
    return await (await code).arrayBuffer();
  }
}

function makeAppEnv(app: App) {
  return {
    taca_RenderingContext_applyBindings(bindings: number) {
      app.bindingApply(bindings);
    },
    taca_RenderingContext_applyPipeline(pipeline: number) {
      app.pipelineApply(pipeline);
    },
    taca_RenderingContext_applyUniforms(uniforms: number) {
      app.uniformsApply(uniforms);
    },
    taca_RenderingContext_beginPass() {},
    taca_RenderingContext_commitFrame() {
      app.frameCommit();
    },
    taca_RenderingContext_draw(
      itemBegin: number,
      itemCount: number,
      instanceCount: number
    ) {
      app.draw(itemBegin, itemCount, instanceCount);
    },
    taca_RenderingContext_drawText(text: number, x: number, y: number) {
      app.drawText(app.readString(text), x, y);
    },
    taca_RenderingContext_drawTexture(texture: number, x: number, y: number) {
      // TODO Source and dest rect? Instanced?
      app.drawTexture(texture, x, y);
    },
    taca_RenderingContext_endPass() {},
    taca_RenderingContext_newBuffer(type: number, info: number) {
      return app.bufferNew(type, info);
    },
    taca_RenderingContext_newPipeline(info: number) {
      return app.pipelineNew(info);
    },
    taca_RenderingContext_newShader(bytes: number) {
      app.shaders.push(shaderNew(app.readBytes(bytes)));
      return app.shaders.length;
    },
    taca_Window_newRenderingContext() {
      // TODO Use this for offscreen contexts for render to texture.
      // TODO Need size? Resizable? Reallocate in same index?
      return 1;
    },
    taca_Window_print(text: number) {
      console.log(app.readString(text));
    },
    taca_Window_setTitle(title: number) {
      // TODO Abstract to provide callbacks for these things?
      document.title = app.readString(title);
    },
    taca_Window_state(result: number) {
      // TODO Include time.
      const { clientWidth, clientHeight } = app.canvas;
      const [pointerX, pointerY] = app.pointerPos;
      const view = app.memoryViewMake(result, 4 * 4);
      setF32(view, 0, pointerX);
      setF32(view, 4, pointerY);
      setF32(view, 8, clientWidth);
      setF32(view, 12, clientHeight);
    },
  };
}

interface Pipeline {
  attributes: AttrInfo[];
  buffers: BufferInfo[];
  program: WebGLProgram;
  uniforms: Uniforms;
}

interface PipelineInfo {
  fragment: ShaderInfo;
  vertex: ShaderInfo;
  vertexAttrs: AttrInfo[];
  vertexBuffers: BufferInfo[];
}

function pipelineInfoDefault(info: Partial<PipelineInfo> = {}): PipelineInfo {
  const fragment: Partial<ShaderInfo> = info.fragment ?? {};
  const vertex: Partial<ShaderInfo> = info.vertex ?? {};
  fragment.entry ||= "fragment_main";
  vertex.entry ||= "vertex_main";
  // The second `|| 1` isn't needed, but it's less risky against reorder.
  fragment.shader ||= vertex.shader || 1;
  vertex.shader ||= fragment.shader || 1;
  return {
    fragment: fragment as ShaderInfo,
    vertex: vertex as ShaderInfo,
    vertexAttrs: info.vertexAttrs ?? [],
    vertexBuffers: info.vertexBuffers ?? [],
  };
}

function setF32(view: DataView, byteOffset: number, value: number) {
  return view.setFloat32(byteOffset, value, true);
}

// function setU32(view: DataView, byteOffset: number, value: number) {
//   return view.setUint32(byteOffset, value, true);
// }

interface ShaderInfo {
  entry: string;
  shader: number;
}

const textDecoder = new TextDecoder();

interface Texture {
  size: [number, number];
  texture: WebGLTexture;
  usedSize: [number, number];
}

interface Uniforms {
  count: number;
  size: number;
  // TODO These are needed only once, not per pipeline.
  tacaIndex: number;
  tacaSize: number;
}
