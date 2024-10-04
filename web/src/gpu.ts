export function findBindGroups(
  gl: WebGL2RenderingContext,
  program: WebGLProgram
) {
  const uniforms = [] as UniformInfo[];
  const blockCount = gl.getProgramParameter(program, gl.ACTIVE_UNIFORM_BLOCKS);
  for (let index = 0; index < blockCount; index += 1) {
    // The block names don't give binding number, but individual uniforms do.
    const uniformIndices = gl.getActiveUniformBlockParameter(
      program,
      index,
      gl.UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES
    ) as Uint32Array;
    if (!uniformIndices.length) continue;
    const uniform = gl.getActiveUniform(program, uniformIndices[0])!;
    const uniformInfo = describeUniform(gl, uniform);
    if (!uniformInfo) continue;
    uniforms.push({ index, location: null, ...uniformInfo });
  }
  const uniformCount = gl.getProgramParameter(program, gl.ACTIVE_UNIFORMS);
  for (let index = 0; index < uniformCount; index += 1) {
    const uniform = gl.getActiveUniform(program, index)!;
    const uniformInfo = describeUniform(gl, uniform);
    if (uniformInfo?.kind != "sampler") continue;
    uniforms.push({
      index,
      location: gl.getUniformLocation(program, uniform.name),
      ...uniformInfo,
    });
  }
  uniforms.sort((a, b) => {
    let result = a.group - a.group;
    if (result) return result;
    return a.binding - b.binding;
  });
  const bindGroups = [] as BindGroupLayout[];
  for (const uniform of uniforms) {
    while (bindGroups.length <= uniform.group) {
      bindGroups.push({ group: bindGroups.length, bindings: [] });
    }
    const bindGroup = bindGroups[uniform.group];
    while (bindGroup.bindings.length < uniform.binding) {
      bindGroup.bindings.push(null);
    }
    bindGroup.bindings.push(uniform);
  }
  return bindGroups;
}

function describeUniform(gl: WebGL2RenderingContext, uniform: WebGLActiveInfo) {
  const [name] = uniform.name.split(".");
  const match = /_group_(\d+)_binding_(\d+)/.exec(name);
  if (!match) return null;
  const isField = uniform.name.indexOf(".") >= 0;
  const group = parseInt(match[1]);
  const binding = parseInt(match[2]);
  const kind =
    uniform.type == gl.SAMPLER_2D ? "sampler" : isField ? "buffer" : null;
  if (!kind) return null;
  return { group, binding, kind } as NonIndexedUniformInfo;
}

export type BindingKind = "buffer" | "sampler";

export interface BindGroupLayout {
  group: number;
  bindings: (UniformInfo | null)[];
}

interface NonIndexedUniformInfo {
  group: number;
  binding: number;
  kind: BindingKind;
}

export interface UniformInfo extends NonIndexedUniformInfo {
  /** Either block index for buffer or uniform index for sampler. */
  index: number;
  location: WebGLUniformLocation | null;
}
