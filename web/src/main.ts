import {
  default as cana,
  lz4Decompress,
  type Shader,
  shaderNew,
  shaderToGlsl,
  ShaderStage,
} from "../pkg/cana";
import { Part, textEncoder } from "./part";
import {
  Texture,
  TexturePipeline,
  fragmentMunge,
  imageDecode,
  shaderMunge,
  shaderProgramBuild,
} from "./drawing";
import { BindGroupLayout, findBindGroups } from "./gpu";
import { keys, keyText } from "./key";
import { fail, getF32, getU32, getU8, setF32, setU32 } from "./util";
import { makeWasiEnv } from "./wasi";
import { unzipSync } from "fflate";

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
    this.addListeners(canvas);
    this.config = config;
    const gl = (this.gl = config.canvas.getContext("webgl2")!);
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    this.texturePipeline = new TexturePipeline(this.gl);
    // Resize will fail if we couldn't get a context.
    this.resizeCanvas();
    // TODO Track for deregistration needs?
    new ResizeObserver(() => (this.resizeNeeded = true)).observe(config.canvas);
  }

  private addListeners(canvas: HTMLCanvasElement) {
    canvas.addEventListener("keydown", (event) => {
      this.keyEventHandle(event, true);
    });
    canvas.addEventListener("keyup", (event) => {
      this.keyEventHandle(event, false);
    });
    const handleMouse = (event: MouseEvent) => {
      if (event.buttons) {
        audioEnsureResumed(this.audioContext);
      }
      const rect = canvas.getBoundingClientRect();
      this.pointerPos = [event.clientX - rect.left, event.clientY - rect.top];
      this.pointerPress = event.buttons;
    };
    canvas.addEventListener("mousedown", (event: MouseEvent) => {
      handleMouse(event);
      this.partsUpdate(eventTypes.press);
    });
    canvas.addEventListener("mouseup", (event: MouseEvent) => {
      handleMouse(event);
      this.partsUpdate(eventTypes.release);
    });
    canvas.addEventListener("mousemove", handleMouse);
    const handleTouch = (event: TouchEvent) => {
      event.preventDefault();
      audioEnsureResumed(this.audioContext);
      // TODO Multitouch?
      const touch = event.touches[0];
      const rect = canvas.getBoundingClientRect();
      this.pointerPos = [touch.clientX - rect.left, touch.clientY - rect.top];
      this.pointerPress = 1;
    };
    canvas.addEventListener("touchend", (event: TouchEvent) => {
      handleTouch(event);
      this.partsUpdate(eventTypes.release);
    });
    canvas.addEventListener("touchstart", (event: TouchEvent) => {
      handleTouch(event);
      this.partsUpdate(eventTypes.press);
    });
    canvas.addEventListener("touchmove", handleTouch);
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
          [gl.FLOAT_VEC3]: [3, gl.FLOAT],
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

  audioContext = new AudioContext();

  bindGroups = [] as {
    pipeline: number;
    group: number;
    buffers: number[];
    samplers: number[];
    textures: number[];
  }[];

  bindingsApply(bindings: number) {
    this.#pipelinedEnsure();
    const { bindGroups, buffers, gl, pipeline, textures } = this;
    // TODO Assert pipeline.
    const bindGroup = bindGroups[bindings - 1];
    const layout = pipeline!.bindGroups[bindGroup.group];
    // TODO Samplers.
    let bufferIndex = 0;
    let textureIndex = 0;
    for (const bindingLayout of layout.bindings) {
      switch (bindingLayout?.kind) {
        case "buffer": {
          const buffer = buffers[bindGroup.buffers[bufferIndex] - 1];
          gl.uniformBlockBinding(
            pipeline!.program,
            bindingLayout.index,
            bufferIndex + 1
          );
          gl.bindBufferBase(
            gl.UNIFORM_BUFFER,
            bufferIndex + 1,
            (buffer as GpuBuffer).buffer
          );
          bufferIndex += 1;
          break;
        }
        case "sampler": {
          const texture = textures[bindGroup.textures[textureIndex] - 1];
          gl.activeTexture(gl.TEXTURE0 + textureIndex);
          gl.bindTexture(gl.TEXTURE_2D, texture.texture);
          gl.uniform1i(bindingLayout.location, textureIndex);
          textureIndex += 1;
          break;
        }
      }
    }
    this.bound = true;
  }

  bindingsNew(part: Part, info: number) {
    const infoBytes = part.memoryViewMake(info, 8 * 4);
    const pipeline = getU32(infoBytes, 0);
    const group = getU32(infoBytes, 4);
    const buffers = part.readAny(info + 8, 4, (view, offset) =>
      getU32(view, offset)
    );
    const samplers = part.readAny(info + 16, 4, (view, offset) =>
      getU32(view, offset)
    );
    const textures = part.readAny(info + 24, 4, (view, offset) =>
      getU32(view, offset)
    );
    // TODO Connecting samplers to textures is hard without more shader digging.
    // TODO Assert just one sampler for now?
    const bindGroup = { pipeline, group, buffers, samplers, textures };
    this.bindGroups.push(bindGroup);
    return this.bindGroups.length;
  }

  bound = false;

  buffersApply(part: Part, buffersPtr: number) {
    const { buffers } = this;
    // Minimize allocations because this is in the draw loop.
    const view = part.memoryView();
    const vertexPtr = getU32(view, buffersPtr);
    const vertexLen = getU32(view, buffersPtr + 4);
    const index = buffers[getU32(view, buffersPtr + 8) - 1];
    // TODO Predefine bindings to avoid allocations?
    const vertex = new Array<Buffer>(vertexLen);
    for (var i = 0; i < vertexLen; i += 1) {
      vertex[i] = buffers[getU32(view, vertexPtr + 4 * i) - 1];
    }
    this.boundBuffers = { index, vertex };
    this.buffered = false;
  }

  boundBuffers: Buffers | null = null;
  boundBuffersDefault: Buffers | null = null;

  bufferNew(part: Part, type: number, info: number) {
    const infoBytes = part.memoryViewMake(info, 2 * 4);
    const ptr = getU32(infoBytes, 0);
    const size = getU32(infoBytes, 4);
    const data = ptr
      ? part.memoryBytes().subarray(ptr, ptr + size)
      : new Uint8Array(size);
    if (type == 3) {
      // Cpu buffer.
      const bytes = new Uint8Array(data.length);
      bytes.set(data);
      this.buffers.push({ bytes, kind: "cpu", length: bytes.length });
    } else {
      // Gpu buffer.
      const { gl } = this;
      const buffer = gl.createBuffer() ?? fail();
      const kind = ["vertex", "index", "uniform"][type] as BufferKind;
      const usage = ptr ? gl.STATIC_DRAW : gl.STREAM_DRAW;
      // TODO Change to numbers for kind or just cache these elsewhere?
      const target =
        [gl.ARRAY_BUFFER, gl.ELEMENT_ARRAY_BUFFER, gl.UNIFORM_BUFFER][type] ??
        fail();
      gl.bindBuffer(target, buffer);
      gl.bufferData(target, data, usage);
      this.buffers.push({ buffer, kind, mutable: !ptr, size });
    }
    return this.buffers.length;
  }

  bufferRead(part: Part, bufferPtr: number, slice: number, offset: number) {
    const buffer = this.buffers[bufferPtr - 1];
    if (buffer.kind != "cpu") {
      // TODO Support reading some gpu buffers.
      return;
    }
    const bytes = part.readBytes(slice);
    const length = Math.min(buffer.bytes.length - offset, bytes.length);
    bytes.set(buffer.bytes.subarray(offset, offset + length));
  }

  bufferUpdate(part: Part, bufferPtr: number, slice: number, offset: number) {
    const bytes = part.readBytes(slice);
    const { buffers, gl } = this;
    const bufferWrapper = buffers[bufferPtr - 1];
    if (bufferWrapper.kind == "cpu") {
      // Cpu buffer.
      const length = Math.max(bufferWrapper.length, offset + bytes.length);
      bufferWrapper.length = length;
      if (length > bufferWrapper.bytes.length) {
        const newBytes = new Uint8Array(Math.ceil(length * 1.5));
        newBytes.set(bufferWrapper.bytes);
        bufferWrapper.bytes = newBytes;
      }
      bufferWrapper.bytes.set(bytes, offset);
    } else {
      // Gpu buffer.
      const { buffer, kind, mutable } = bufferWrapper;
      // TODO Is this checked automatically?
      mutable || fail();
      const target = {
        vertex: gl.ARRAY_BUFFER,
        index: gl.ELEMENT_ARRAY_BUFFER,
        uniform: gl.UNIFORM_BUFFER,
      }[kind];
      gl.bindBuffer(target, buffer);
      gl.bufferSubData(target, offset, bytes);
    }
  }

  buffered = false;

  #bufferedEnsure() {
    if (!this.buffered) {
      this.#pipelinedEnsure();
      if (!this.boundBuffers) {
        let { boundBuffersDefault } = this;
        if (!this.boundBuffersDefault) {
          const { buffers } = this;
          const find = (kind: string) =>
            buffers.find((buffer) => (buffer as GpuBuffer).kind == kind) ??
            fail();
          this.boundBuffersDefault = boundBuffersDefault = {
            index: find("index"),
            vertex: [find("vertex")],
          };
        }
        this.boundBuffers = boundBuffersDefault;
      }
      this.#buffersBind();
    }
    if (!this.bound && this.bindGroups.length) {
      this.bindingsApply(1);
    }
  }

  buffers: Buffer[] = [];

  #buffersBind() {
    // If at least two buffers, presumes one is data and one index.
    const { boundBuffers, gl, pipeline } = this;
    // Vertex buffer.
    const attrs = pipeline!.attributes;
    const vertexBuffers = boundBuffers!.vertex;
    const vertexBufferInfos = pipeline!.buffers;
    let vertexBufferIndex = -1;
    let vertex: Buffer;
    let vertexInfo: BufferInfo;
    let nextAttrIndex = -1;
    let stride = 0;
    for (let a = 0; a < attrs.length; a += 1) {
      if (a >= nextAttrIndex) {
        // Move forward in buffers.
        vertexBufferIndex += 1;
        vertex = vertexBuffers[vertexBufferIndex];
        vertexInfo = vertexBufferInfos[vertexBufferIndex];
        nextAttrIndex =
          vertexBufferInfos[vertexBufferIndex + 1]?.firstAttr ??
          Number.MAX_SAFE_INTEGER;
        // Work out drawing.
        gl.bindBuffer(gl.ARRAY_BUFFER, (vertex as GpuBuffer).buffer);
        stride = vertexInfo.stride;
      }
      const attr = attrs[a];
      const { loc, offset, size, type } = attr;
      gl.enableVertexAttribArray(loc);
      gl.vertexAttribPointer(loc, size, type, false, stride, offset);
      gl.vertexAttribDivisor(loc, vertexInfo!.step ? 1 : 0);
    }
    // TODO Instance buffer.
    // Index buffer.
    const index = boundBuffers!.index;
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, (index as GpuBuffer).buffer);
    this.buffered = true;
  }

  canvas: HTMLCanvasElement;

  config: AppConfig;

  draw(itemBegin: number, itemCount: number, instanceCount: number) {
    // console.log(`draw(${itemBegin}, ${itemCount}, ${instanceCount})`);
    this.#bufferedEnsure();
    const { gl } = this;
    gl.drawElementsInstanced(
      gl.TRIANGLES,
      itemCount,
      gl.UNSIGNED_SHORT,
      itemBegin,
      instanceCount
    );
  }

  drawText(text: string, x: number, y: number) {
    if (!text) return;
    if (text != this.textTextureText) {
      // TODO Consider font, color, and so on.
      // TODO LRU cache on atlas as separate helper library?
      this.textTexture = this.textDraw(text, this.textTexture || undefined);
      this.textTextureText = text;
    }
    const {
      usedSize: [sizeX, sizeY],
    } = this.textures[this.textTexture - 1];
    const [alignX, alignY] = this.textAlignVals;
    // TODO Other alignments.
    switch (alignX) {
      case "left": {
        x += sizeX / 2;
        break;
      }
      case "right": {
        x -= sizeX / 2;
        break;
      }
    }
    switch (alignY) {
      case "alphabetic":
      case "bottom": {
        y -= sizeY / 2;
        break;
      }
      case "top": {
        y += sizeY / 2;
        break;
      }
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

  frameCommit() {
    this.bound = this.buffered = this.passBegun = false;
    this.boundBuffers = this.pipeline = null;
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

  keyEvent = new DataView(new Uint32Array(3).buffer);
  keyEventBytes = new Uint8Array(this.keyEvent.buffer);

  keyEventHandle(event: KeyboardEvent, pressed: boolean) {
    switch (event.code) {
      // TODO What other cases?
      case "KeyR":
        if (event.ctrlKey) {
          return;
        }
        break;
      case "F11":
      case "F12":
        return;
    }
    event.preventDefault();
    audioEnsureResumed(this.audioContext);
    if (event.repeat) return;
    const { buffers, keyEvent, textEvent } = this;
    const text = pressed ? keyText(event) : "";
    if (text) {
      const bytes = textEncoder.encode(text);
      if (this.textBuffer) {
        const buffer = buffers[this.textBuffer - 1] as CpuBuffer;
        if (buffer.bytes.length < bytes.length) {
          buffer.bytes = bytes;
        } else {
          buffer.bytes.set(bytes);
        }
        buffer.length = bytes.length;
      } else {
        buffers.push({
          bytes,
          kind: "cpu",
          length: bytes.length,
        });
        this.textBuffer = buffers.length;
      }
    }
    // TODO Combine text into key event again?
    setU32(keyEvent, 0, pressed ? 1 : 0);
    setU32(keyEvent, 4, keys[event.code] ?? 0);
    setU32(keyEvent, 8, 0);
    this.partsUpdate(eventTypes.key);
    if (text) {
      const textBuffer = this.textBuffer;
      setU32(textEvent, 0, textBuffer);
      setU32(textEvent, 4, (buffers[textBuffer - 1] as CpuBuffer).length);
      this.partsUpdate(eventTypes.text);
    }
  }

  gl: WebGL2RenderingContext;

  imageDecode(part: Part, bytes: number) {
    const { gl, textures } = this;
    let pointer = 0;
    const texture = imageDecode(
      gl,
      part.readBytes(bytes),
      () => this.taskFinish(),
      (reason) => {
        this.taskFinish();
        fail(reason);
      }
    );
    this.tasksActive += 1;
    textures.push(texture);
    pointer = textures.length;
    return pointer;
  }

  indexBuffer: Buffer | null = null;

  offscreen = new OffscreenCanvas(1, 1);
  offscreenContext = this.offscreen.getContext("2d") ?? fail();

  parts: Part[] = [];

  partsUpdate(kind: number) {
    for (const part of this.parts) {
      part.update(kind);
    }
  }

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
    (pipeline.depthTest ? gl.enable : gl.disable).call(gl, gl.DEPTH_TEST);
    gl.useProgram(pipeline.program);
    this.tacaBufferEnsure();
    // Presume we need new buffers bound when the program changes.
    this.buffered = false;
  }

  #pipelineBuild(pipelineInfo: PipelineInfo) {
    // console.log(pipelineInfo);
    const { gl, pipelines, shaders } = this;
    const shaderMake = (info: ShaderInfo, stage: ShaderStage) =>
      shaderToGlsl(shaders[info.shader - 1], stage, info.entry);
    const vertex = shaderMunge(
      shaderMake(pipelineInfo.vertex, ShaderStage.Vertex)
    );
    const fragment = shaderMunge(
      fragmentMunge(shaderMake(pipelineInfo.fragment, ShaderStage.Fragment))
    );
    // console.log(vertex);
    // console.log(fragment);
    const program = shaderProgramBuild(gl, vertex, fragment);
    const bindGroups = findBindGroups(gl, program);
    pipelineInfo = this.#attributesBuild(program, pipelineInfo);
    const uniforms = this.#uniformsBuild(program);
    pipelines.push({
      attributes: pipelineInfo.vertexAttrs,
      bindGroups,
      buffers: pipelineInfo.vertexBuffers,
      depthTest: pipelineInfo.depthTest,
      program,
      uniforms,
    });
    // console.log(pipelineInfo);
    // console.log(this.pipelines);
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

  pipelineNew(part: Part, info: number) {
    let pipelineInfo = this.pipelineInfoRead(part, info);
    pipelineInfo = this.#pipelineBuild(pipelineInfo);
    return this.pipelines.length;
  }

  private pipelineInfoRead(part: Part, info: number): PipelineInfo {
    // TODO Can wit-bindgen or flatbuffers automate some of this?
    const infoView = part.memoryViewMake(info, 11 * 4);
    const readShaderInfo = (offset: number) => {
      return {
        entry: part.readString(infoView.byteOffset + offset),
        shader: getU32(infoView, offset + 2 * 4),
      };
    };
    const pipelineInfo: PipelineInfo = {
      depthTest: !!getU8(infoView, 0),
      fragment: readShaderInfo(1 * 4),
      vertex: readShaderInfo(4 * 4),
      vertexAttrs: part.readAny(
        info + 7 * 4,
        2 * 4,
        (view, offset): AttrInfo => ({
          count: 1,
          loc: getU32(view, offset),
          offset: getU32(view, offset + 1 * 4),
          size: 0,
          type: 0,
        })
      ),
      vertexBuffers: part.readAny(
        info + 9 * 4,
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
  pointerPress = 0;

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
  sounds: Sound[] = [];

  soundDecode(part: Part, bytes: number) {
    const { audioContext, sounds } = this;
    const sound = { buffer: null } as Sound;
    sounds.push(sound);
    const pointer = sounds.length;
    audioContext.decodeAudioData(
      part.readBytes(bytes).slice().buffer,
      (buffer) => {
        sound.buffer = buffer;
        // console.log(buffer.duration);
        this.taskFinish();
      },
      (err) => {
        this.taskFinish();
        fail(err.message);
      }
    );
    this.tasksActive += 1;
    return pointer;
  }

  soundPlay(part: Part, info: number) {
    const infoView = part.memoryViewMake(info, 6 * 4);
    const sound = getU32(infoView, 0 * 4);
    const { audioContext, sounds } = this;
    const source = audioContext.createBufferSource();
    source.buffer = sounds[sound - 1].buffer;
    // Delay.
    const delay = getF32(infoView, 1 * 4);
    const startTime = delay > 0 ? audioContext.currentTime + delay : 0;
    // Rate.
    const rate = getF32(infoView, 2 * 4);
    switch (getU32(infoView, 3 * 4)) {
      case 0:
        source.detune.value = 100 * rate;
        break;
      case 1:
        source.playbackRate.value = rate;
        break;
    }
    // Volume.
    let volume = getF32(infoView, 4 * 4);
    if (getU32(infoView, 5 * 4) == 0) {
      // Decibels.
      volume = Math.pow(10, volume / 20);
    }
    if (volume == 1) {
      source.connect(audioContext.destination);
    } else {
      const gainNode = audioContext.createGain();
      gainNode.gain.value = volume;
      source.connect(gainNode);
      gainNode.connect(audioContext.destination);
    }
    // Play.
    source.start(startTime);
    return 0;
  }

  tacaBuffer: WebGLBuffer | null = null;

  private tacaBufferEnsure() {
    const { gl, pipeline } = this;
    if (!this.tacaBuffer) {
      const tacaSize = pipeline!.uniforms.tacaSize;
      const tacaBuffer = gl.createBuffer() ?? fail();
      gl.bindBuffer(gl.UNIFORM_BUFFER, tacaBuffer);
      gl.bufferData(gl.UNIFORM_BUFFER, tacaSize, gl.STREAM_DRAW);
      gl.bindBufferBase(gl.UNIFORM_BUFFER, 0, tacaBuffer);
      this.tacaBuffer = tacaBuffer;
      this.tacaBufferUpdate();
    }
  }

  private tacaBufferUpdate() {
    const { canvas } = this.config;
    if (this.tacaBuffer) {
      const { gl } = this;
      for (const pipeline of this.pipelines) {
        // This helps flip the y axis to match wgpu.
        // TODO Instead render to texture then flip the texture.
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

  tasksActive = 0;

  taskFinish() {
    this.tasksActive -= 1;
    this.partsUpdate(eventTypes.tasksDone);
  }

  textAlign(x: number, y: number) {
    this.textAlignVals = [
      ([undefined, "center", "right"][x] ?? "left") as CanvasTextAlign,
      ([undefined, "top", "middle", "bottom"][y] ??
        "alphabetic") as CanvasTextBaseline,
    ];
  }

  textAlignVals: [CanvasTextAlign, CanvasTextBaseline] = ["left", "alphabetic"];
  textBuffer = 0;

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

  textEvent = new DataView(new Uint32Array(2).buffer);
  textEventBytes = new Uint8Array(this.textEvent.buffer);

  textTexture: number = 0;
  textTextureText: string = "";
  texturePipeline: TexturePipeline;

  textureInfo(part: Part, result: number, texture: number) {
    let size = this.textures[texture - 1]?.size ?? [0, 0];
    const view = part.memoryViewMake(result, 2 * 4);
    setF32(view, 0, size[0]);
    setF32(view, 4, size[1]);
  }

  textures: Texture[] = [];

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
      const isTaca = name == "taca_uniform_block";
      if (isTaca) {
        tacaIndex = i;
        tacaSize = nextSize;
      } else {
        // The idea here was expecting only one but repeated as fs vs vs, like
        // naga likes to do, but it doesn't accommodate actual multiple.
        // TODO Replace all this once we move to pure bind groups.
        // if (size > 0 && nextSize != size) fail();
        size = nextSize;
      }
      gl.uniformBlockBinding(program, i, isTaca ? 0 : i + 1);
    }
    return { count, size, tacaIndex, tacaSize };
  }

  vertexBuffer: Buffer | null = null;

  windowState(part: Part, result: number) {
    // TODO Include time.
    const { clientWidth, clientHeight } = this.canvas;
    const [pointerX, pointerY] = this.pointerPos;
    const view = part.memoryViewMake(result, 5 * 4);
    setF32(view, 0, pointerX);
    setF32(view, 4, pointerY);
    setU32(view, 8, this.pointerPress);
    setF32(view, 12, clientWidth);
    setF32(view, 16, clientHeight);
  }
}

interface AttrInfo {
  count: number;
  loc: number;
  offset: number;
  size: number;
  type: number;
}

async function audioEnsureResumed(audioContext: AudioContext) {
  if (audioContext.state == "suspended") {
    await audioContext.resume();
    console.log(`AudioContext state: ${audioContext.state}`);
  }
}

interface Buffers {
  // TODO images/textures
  index: Buffer;
  vertex: Buffer[];
}

type Buffer = CpuBuffer | GpuBuffer;

interface CpuBuffer {
  bytes: Uint8Array;
  kind: "cpu";
  length: number;
}

interface GpuBuffer {
  buffer: WebGLBuffer;
  kind: BufferKind;
  mutable: boolean;
  size: number;
}

interface BufferInfo {
  firstAttr: number;
  step: number;
  stride: number;
}

type BufferKind = "vertex" | "index" | "uniform";

const eventTypes = {
  frame: 0,
  key: 1,
  tasksDone: 2,
  press: 3,
  release: 4,
  text: 5,
};

async function loadApp(config: AppConfig) {
  const appData = config.code as ArrayBuffer;
  config.code = undefined;
  // Read wasm buffers.
  const appBytes = new Uint8Array(appData);
  const wasmBuffers =
    appBytes[0] == 0x04
      ? // Presume lz4 because wasm starts with 0.
        [lz4Decompress(appBytes).buffer]
      : appBytes[0] == 0x50
      ? zipRead(appBytes)
      : [appData];
  // Instantiate extensions.
  // TODO Recursive dependencies.
  const app = new App(config);
  const reservedExports = new Set(["init", "initialize", "start", "update"]);
  const bonusExports = {};
  const makePart = async (buffer: ArrayBufferLike) => {
    const part = new Part();
    const env = makeAppEnv(app, part);
    Object.assign(env, bonusExports);
    const wasi = makeWasiEnv(part);
    let { instance } = await WebAssembly.instantiate(buffer, {
      env,
      wasi_snapshot_preview1: wasi,
    });
    part.init(instance);
    return part;
  };
  const extPromises = wasmBuffers.slice(0, -1).map(makePart);
  const parts = await Promise.all(extPromises);
  // Fill in bonus exports after extensions are instantiated.
  for (const part of parts) {
    Object.assign(
      bonusExports,
      Object.fromEntries(
        Object.entries(part.exports).filter(([key]) => {
          const reserved =
            reservedExports.has(key) || // core reserved
            key.includes(".") || // reserved for future namespacing rules
            key.startsWith("_") || // reserved for whatever
            key.startsWith("taca_"); // reserved for taca
          return !reserved;
        })
      )
    );
  }
  // Build the main app part and init everything.
  const appPart = await makePart(wasmBuffers.at(-1)!);
  parts.push(appPart);
  // Init them in order.
  for (const part of parts) {
    const exports = part.exports;
    if (exports._initialize) {
      exports._initialize();
    } else if (exports._start) {
      exports._start();
    }
    if (exports.start) {
      exports.start();
    }
  }
  // Run updates.
  app.parts = parts;
  if (parts.some((it) => it.exports.update)) {
    const update = () => {
      try {
        app.partsUpdate(eventTypes.frame);
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

function makeAppEnv(app: App, part: Part) {
  return {
    taca_bindings_apply(bindings: number) {
      app.bindingsApply(bindings);
    },
    taca_bindings_new(info: number) {
      return app.bindingsNew(part, info);
    },
    taca_buffer_new(type: number, info: number) {
      return app.bufferNew(part, type, info);
    },
    taca_buffer_read(buffer: number, bytes: number, offset: number) {
      return app.bufferRead(part, buffer, bytes, offset);
    },
    taca_buffer_update(buffer: number, bytes: number, offset: number) {
      return app.bufferUpdate(part, buffer, bytes, offset);
    },
    taca_buffers_apply(buffers: number) {
      app.buffersApply(part, buffers);
    },
    taca_clip(x: number, y: number, sizeX: number, sizeY: number) {
      const { gl } = app;
      gl.enable(gl.SCISSOR_TEST);
      gl.scissor(
        Math.round(x),
        Math.round(y),
        Math.round(sizeX),
        Math.round(sizeY)
      );
    },
    taca_draw(itemBegin: number, itemCount: number, instanceCount: number) {
      app.draw(itemBegin, itemCount, instanceCount);
    },
    taca_image_decode(bytes: number) {
      return app.imageDecode(part, bytes);
    },
    taca_key_event(result: number) {
      part.memoryBytes().set(app.keyEventBytes, result);
    },
    taca_pipeline_apply(pipeline: number) {
      app.pipelineApply(pipeline);
    },
    taca_pipeline_new(info: number) {
      return app.pipelineNew(part, info);
    },
    taca_print(text: number) {
      console.log(part.readString(text));
    },
    taca_shader_new(bytes: number) {
      app.shaders.push(shaderNew(part.readBytes(bytes)));
      return app.shaders.length;
    },
    taca_sound_decode(bytes: number) {
      return app.soundDecode(part, bytes);
    },
    taca_sound_play(info: number) {
      return app.soundPlay(part, info);
    },
    taca_text_align(x: number, y: number) {
      app.textAlign(x, y);
    },
    taca_text_draw(text: number, x: number, y: number) {
      app.drawText(part.readString(text), x, y);
    },
    taca_text_event(result: number) {
      part.memoryBytes().set(app.textEventBytes, result);
    },
    taca_texture_info(result: number, texture: number) {
      app.textureInfo(part, result, texture);
    },
    taca_title_update(title: number) {
      // TODO Abstract to provide callbacks for these things?
      document.title = part.readString(title);
    },
    taca_window_state(result: number) {
      app.windowState(part, result);
    },
  };
}

interface Pipeline {
  attributes: AttrInfo[];
  bindGroups: BindGroupLayout[];
  buffers: BufferInfo[];
  depthTest: boolean;
  program: WebGLProgram;
  uniforms: Uniforms;
}

interface PipelineInfo {
  depthTest: boolean;
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
    depthTest: info.depthTest ?? false,
    fragment: fragment as ShaderInfo,
    vertex: vertex as ShaderInfo,
    vertexAttrs: info.vertexAttrs ?? [],
    vertexBuffers: info.vertexBuffers ?? [],
  };
}

interface ShaderInfo {
  entry: string;
  shader: number;
}

interface Sound {
  buffer: AudioBuffer | null;
}

interface Uniforms {
  count: number;
  size: number;
  // TODO These are needed only once, not per pipeline.
  tacaIndex: number;
  tacaSize: number;
}

function zipRead(bytes: Uint8Array) {
  const entries = unzipSync(bytes, {
    // Extract only core taca files to start with. TODO Be more selective?
    filter: (info) => /^(?:ext\/)?[^/]+\.wasm/.test(info.name),
  });
  // console.log(entries);
  const parts = Object.entries(entries)
    .filter((it) => it[0].startsWith("ext/"))
    .sort((a, b) => (a[0] > b[0] ? 1 : -1))
    .map((it) => it[1].buffer);
  parts.push(entries["app.wasm"].buffer);
  // console.log(parts);
  return parts;
}
