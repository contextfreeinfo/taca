```
./build.sh
```

Also

```
dxc -T vs_6_0 -E vs_main -spirv -Fo src/vs.spv src/shader.hlsl
dxc -T ps_6_0 -E fs_main -spirv -Fo src/fs.spv src/shader.hlsl
```
