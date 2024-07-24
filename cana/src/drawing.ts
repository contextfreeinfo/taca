import { fail } from "./util";

export class TexturePipeline {
  constructor(gl: WebGL2RenderingContext) {
    this.gl = gl;
    const program = shaderProgramBuild(gl, textureVertSource, textureFragSource);
    this.program = program;
    this.drawInfoBuffer = gl.createBuffer() ?? fail();
    this.drawInfoIndex = gl.getUniformBlockIndex(program, "drawInfo") ?? fail();
    gl.uniformBlockBinding(program, this.drawInfoIndex, 0);
    this.sampler = gl.getUniformLocation(program, "sampler") ?? fail();
    const vertexArray = gl.createVertexArray() ?? fail();
    this.vertexArray = vertexArray;
    const indices = new Uint16Array([0, 1, 2, 0, 2, 3]);
    gl.bindVertexArray(vertexArray);
    // Vertex position and tex coords.
    [
      [1, 1, -1, 1, -1, -1, 1, -1],
      [1, 1, 0, 1, 0, 0, 1, 0],
    ].forEach((array, i) => {
      const buffer = gl.createBuffer() ?? fail();
      gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
      gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(array), gl.STATIC_DRAW);
      gl.vertexAttribPointer(i, 2, gl.FLOAT, false, 0, 0);
      gl.enableVertexAttribArray(i);
    });
    // Index.
    const indexBuffer = gl.createBuffer() ?? fail();
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, indices, gl.STATIC_DRAW);
    // Done.
    gl.bindVertexArray(null);
  }

  draw(
    texture: WebGLTexture,
    canvasWidth: number,
    canvasHeight: number,
    x: number,
    y: number,
    width: number,
    height: number
  ) {
    const { drawInfoBuffer, gl, program, sampler, vertexArray } = this;
    const drawInfoArray = new Float32Array([
      canvasWidth,
      canvasHeight,
      x,
      y,
      width,
      height,
      0,
      0,
    ]);
    gl.useProgram(program);
    // For fixed system things, vertex array objects are probably fine.
    gl.bindVertexArray(vertexArray);
    try {
      gl.bindBuffer(gl.UNIFORM_BUFFER, drawInfoBuffer);
      gl.bufferData(gl.UNIFORM_BUFFER, drawInfoArray, gl.STREAM_DRAW);
      gl.bindBufferBase(gl.UNIFORM_BUFFER, 0, drawInfoBuffer);
      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, texture);
      gl.uniform1i(sampler, 0);
      gl.drawElements(gl.TRIANGLES, 6, gl.UNSIGNED_SHORT, 0);
    } finally {
      gl.bindVertexArray(null);
    }
  }

  drawInfoBuffer: WebGLBuffer;
  drawInfoIndex: number; // TODO Need to use this?
  gl: WebGL2RenderingContext;
  program: WebGLProgram;
  sampler: WebGLUniformLocation;
  vertexArray: WebGLVertexArrayObject;
}

export function shaderProgramBuild(
  gl: WebGL2RenderingContext,
  vertex: string,
  fragment: string
) {
  const program = gl.createProgram() ?? fail();
  const addShader = (type: number, source: string) => {
    const shader = gl.createShader(type) ?? fail();
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    gl.getShaderParameter(shader, gl.COMPILE_STATUS) ??
      fail(gl.getShaderInfoLog(shader));
    gl.attachShader(program, shader);
  };
  addShader(gl.VERTEX_SHADER, vertex);
  addShader(gl.FRAGMENT_SHADER, fragment);
  gl.linkProgram(program);
  gl.getProgramParameter(program, gl.LINK_STATUS) ??
    fail(gl.getProgramInfoLog(program));
  return program;
}

const textureVertSource = `#version 300 es
layout(location = 0) in vec2 framePos;
layout(location = 1) in vec2 texCoord;
uniform drawInfo {
  vec2 canvasSize;
  vec2 drawPos;
  vec2 drawSize;
};
out vec2 vTexCoord;
void main() {
  vec2 pos = framePos * drawSize * 0.5 + drawPos;
  pos.y = canvasSize.y - pos.y;
  pos = (pos / canvasSize) * 2.0 - 1.0;
  gl_Position = vec4(pos, 0.0, 1.0);
  vTexCoord = texCoord;
}
`;

const textureFragSource = `#version 300 es
precision mediump float;
in vec2 vTexCoord;
out vec4 outColor;
uniform sampler2D sampler;
void main() {
  outColor = texture(sampler, vTexCoord);
}
`;
