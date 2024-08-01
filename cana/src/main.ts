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
  wasm?: ArrayBuffer | Promise<ArrayBuffer>;
}

export async function runApp(config: AppConfig) {
  const [appData] = await Promise.all([config.wasm ?? loadAppData(), cana()]);
  if (appData) {
    await loadApp({ ...config, wasm: appData });
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
    this.config = config;
    this.gl = config.canvas.getContext("webgl2")!;
    this.texturePipeline = new TexturePipeline(this.gl);
    // Resize will fail if we couldn't get a context.
    this.resizeCanvas();
    // TODO Track for deregistration needs?
    new ResizeObserver(() => (this.resizeNeeded = true)).observe(config.canvas);
  }

  #attributesBuild(program: WebGLProgram) {
    const attributes: Attribute[] = [];
    const { gl } = this;
    const attribCount = gl.getProgramParameter(program, gl.ACTIVE_ATTRIBUTES);
    for (let i = 0; i < attribCount; i += 1) {
      const info = gl.getActiveAttrib(program, i) ?? fail();
      const loc = gl.getAttribLocation(program, info.name);
      // const type = {
      //   [gl.FLOAT_VEC2]: "vec2f",
      //   [gl.FLOAT_VEC4]: "vec4f",
      // }[info.type];
      attributes.push({ count: info.size, loc, type: info.type });
    }
    attributes.sort((a, b) => a.loc - b.loc);
    // TODO Buffer handling doesn't belong here in program construction!
    // TODO Stop using vertex array objects because they have rather limited bindings available.
    this.#vertexArrayCreate(attributes);
    // console.log(attributes);
    return attributes;
  }

  bufferNew(type: number, info: number) {
    const infoBytes = this.memoryView(info, 3 * 4);
    const ptr = getU32(infoBytes, 0);
    const size = getU32(infoBytes, 4);
    const itemSize = getU32(infoBytes, 8);
    // TODO Null ptr -> zero buffer for writing.
    const data = this.memoryBytes().subarray(ptr, ptr + size);
    const { gl } = this;
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    const buffer = gl.createBuffer();
    buffer || fail();
    this.buffers.push({
      buffer: buffer!,
      itemSize,
      kind: ["vertex", "index"][type] as "vertex" | "index",
    });
    const target = [gl.ARRAY_BUFFER, gl.ELEMENT_ARRAY_BUFFER][type] ?? fail();
    gl.bindBuffer(target, buffer);
    const usageValue = ptr ? gl.STATIC_DRAW : gl.STREAM_DRAW;
    gl.bufferData(target, data, usageValue);
    return this.buffers.length;
  }

  buffers: Buffer[] = [];

  canvas: HTMLCanvasElement;

  config: AppConfig;

  draw(itemBegin: number, itemCount: number, instanceCount: number) {
    this.#pipelinedEnsure();
    const { gl } = this;
    if (!this.vertexArray) {
      this.vertexArray = this.vertexArrays[0] ?? fail();
      gl.bindVertexArray(this.vertexArray);
    }
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
    this.passBegun = false;
    this.pipeline = this.vertexArray = null;
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

  init(instance: WebAssembly.Instance) {
    this.exports = instance.exports as any;
    this.memory = instance.exports.memory as any;
  }

  memory: WebAssembly.Memory = undefined as any;

  #memoryBuffer: ArrayBuffer | null = null;
  #memoryBufferBytes: Uint8Array | null = null;

  memoryBytes() {
    if (this.#memoryBuffer != this.memory.buffer) {
      // Either on first access or on internal reallocation.
      this.#memoryBuffer = this.memory.buffer;
      this.#memoryBufferBytes = new Uint8Array(this.#memoryBuffer);
    }
    return this.#memoryBufferBytes!;
  }

  memoryView(ptr: number, len: number) {
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
  }

  #pipelineEnsure() {
    const {
      gl,
      pipelines,
      shaders: [shader],
    } = this;
    if (pipelines.length) return;
    const vertex = shaderToGlsl(shader, ShaderStage.Vertex, "vertex_main");
    const fragment = shaderToGlsl(
      shader,
      ShaderStage.Fragment,
      "fragment_main"
    );
    const program = shaderProgramBuild(gl, vertex, fragment);
    // console.log(vertex);
    // console.log(fragment);
    const attributes = this.#attributesBuild(program);
    const uniforms = this.#uniformsBuild(program);
    pipelines.push({ attributes, program, uniforms });
  }

  #pipelinedEnsure() {
    if (!this.pipeline) {
      this.#pipelineEnsure();
      if (!this.passBegun) this.passBegin();
      if (this.pipelines.length == 1) this.pipelineApply(1);
    }
  }

  pipelines: Pipeline[] = [];

  pointerPos: [x: number, y: number] = [0, 0];

  readBytes(spanPtr: number) {
    // Can cache memory bytes when no app calls are being made.
    const memoryBytes = this.memoryBytes();
    const spanView = new DataView(memoryBytes.buffer, spanPtr, 2 * 4);
    // Wasm is explicitly little-endian.
    const contentPtr = getU32(spanView, 0);
    const contentLen = getU32(spanView, 4);
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
      gl.RGBA,
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
    let size = 0;
    let tacaIndex = 0;
    let tacaSize = 0;
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
        if (i > 0 && nextSize != size) fail();
        size = nextSize;
      }
      gl.uniformBlockBinding(program, i, i + 1);
    }
    return { count, size, tacaIndex, tacaSize };
  }

  vertexArray: WebGLVertexArrayObject | null = null;

  #vertexArrayCreate(attributes: Attribute[]) {
    const { buffers, gl } = this;
    if (buffers.length == 2) {
      const vertexArray = gl.createVertexArray() ?? fail();
      gl.bindVertexArray(vertexArray);
      try {
        // Vertex buffer.
        const vertex =
          buffers.find((buffer) => buffer.kind == "vertex") ?? fail();
        gl.bindBuffer(gl.ARRAY_BUFFER, vertex.buffer);
        let offset = 0;
        for (const attr of attributes) {
          const { loc } = attr;
          gl.enableVertexAttribArray(loc);
          const [size, type] =
            {
              [gl.FLOAT_VEC2]: [2, gl.FLOAT],
              [gl.FLOAT_VEC4]: [4, gl.FLOAT],
            }[attr.type] ?? fail();
          const typeSize = { [gl.FLOAT]: 4 }[type] ?? fail();
          // Pad for alignment.
          offset = Math.ceil(offset / typeSize) * typeSize;
          // console.log(size, vertex.itemSize, offset);
          // TODO Item size vs alignment seems very off.
          let { itemSize } = vertex;
          gl.vertexAttribPointer(loc, size, type, false, itemSize, offset);
          offset += size * typeSize;
        }
        // TODO Instance buffer.
        // Index buffer.
        const index =
          buffers.find((buffer) => buffer.kind == "index") ?? fail();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, index.buffer);
      } finally {
        gl.bindVertexArray(null);
      }
      this.vertexArrays.push(vertexArray);
    }
  }

  vertexArrays: WebGLVertexArrayObject[] = [];
}

interface AppExports {
  listen: ((event: number) => void) | undefined;
  _start: () => void;
}

interface Attribute {
  count: number;
  loc: number;
  type: number;
}

interface Buffer {
  buffer: WebGLBuffer;
  itemSize: number;
  kind: "index" | "vertex";
}

function getU32(view: DataView, byteOffset: number) {
  return view.getUint32(byteOffset, true);
}

async function loadApp(config: AppConfig) {
  const appData = config.wasm as ArrayBuffer;
  config.wasm = undefined;
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

async function loadAppData() {
  const url = new URL(window.location.href);
  const params = new URLSearchParams(url.search);
  const app = params.get("app");
  if (app) {
    return await (await fetch(app)).arrayBuffer();
  }
}

function makeAppEnv(app: App) {
  return {
    taca_RenderingContext_applyBindings(bindings: number) {
      // TODO Bindings.
    },
    taca_RenderingContext_applyPipeline(pipeline: number) {
      // TODO Pipeline.
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
      // TODO Source and dest rect?
      app.drawTexture(texture, x, y);
    },
    taca_RenderingContext_endPass() {},
    taca_RenderingContext_newBuffer(type: number, info: number) {
      return app.bufferNew(type, info);
    },
    taca_RenderingContext_newPipeline(bytes: number) {
      // TODO Pipeline.
      console.log("taca_RenderingContext_newPipeline");
    },
    taca_RenderingContext_newShader(bytes: number) {
      app.shaders.push(shaderNew(app.readBytes(bytes)));
      return app.shaders.length;
    },
    taca_Window_newRenderingContext() {
      // TODO If we only have one, we don't need it at all, right?
      // TODO Except context for render to texture.
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
      const view = app.memoryView(result, 4 * 4);
      setF32(view, 0, pointerX);
      setF32(view, 4, pointerY);
      setF32(view, 8, clientWidth);
      setF32(view, 12, clientHeight);
    },
  };
}

interface Pipeline {
  attributes: Attribute[];
  program: WebGLProgram;
  uniforms: Uniforms;
}

function setF32(view: DataView, byteOffset: number, value: number) {
  return view.setFloat32(byteOffset, value, true);
}

// function setU32(view: DataView, byteOffset: number, value: number) {
//   return view.setUint32(byteOffset, value, true);
// }

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
