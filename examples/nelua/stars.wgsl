// Translated to wgsl from: https://www.shadertoy.com/view/MtcGDf

const FLIGHT_SPEED = 8.0;

const DRAW_DISTANCE = 60.0; // Lower this to increase framerate
const FADEOUT_DISTANCE = 10.0; // must be < DRAW_DISTANCE
const FIELD_OF_VIEW = 1.05;

const STAR_SIZE = 0.6; // must be > 0 and < 1
const STAR_CORE_SIZE = 0.14;

const CLUSTER_SCALE = 0.02;
const STAR_THRESHOLD = 0.775;

const BLACK_HOLE_CORE_RADIUS = 0.2;
const BLACK_HOLE_THRESHOLD = 0.9995;
const BLACK_HOLE_DISTORTION = 0.03;

// TODO Uniforms!
const iResolution = vec2f(1920.0, 1080.0);
const iTime = 0.0;

// // http://lolengine.net/blog/2013/07/27/rgb-to-hsv-in-glsl
// vec3 hsv2rgb(vec3 c) {
//     vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
//     vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
//     return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
// }

// https://stackoverflow.com/questions/4200224/random-noise-functions-for-glsl
fn rand(co: vec2f) -> f32 {
    return fract(sin(dot(co.xy, vec2f(12.9898, 78.233))) * 43758.5453);
}

fn getRayDirection(frag_coord: vec2f, cameraDirection: vec3f) -> vec3f {
    let uv = frag_coord.xy / iResolution.xy;

    let screenWidth = 1.0;
    let originToScreen = screenWidth / 2.0 / tan(FIELD_OF_VIEW / 2.0);

    let screenCenter = originToScreen * cameraDirection;
    let baseX = normalize(cross(screenCenter, vec3f(0.0, -1.0, 0.0)));
    let baseY = normalize(cross(screenCenter, baseX));

    return normalize(screenCenter + (uv.x - 0.5) * baseX + (uv.y - 0.5) * iResolution.y / iResolution.x * baseY);
}

fn getDistance(chunkPath: vec3i, localStart: vec3f, localPosition: vec3f) -> f32 {
    return length(vec3f(chunkPath) + localPosition - localStart);
}

fn movePos(localPosition: ptr<function, vec3f>, rayDirection: vec3f, directionBound: vec3f) {
    let directionSign = sign(rayDirection);
    let amountVector = (directionBound - directionSign * *localPosition) / abs(rayDirection);

    let amount = min(amountVector.x, min(amountVector.y, amountVector.z));

    (*localPosition) += amount * rayDirection;
}

// Makes sure that each component of localPosition is >= 0 and <= 1
fn moveInsideBox(
    localPosition: ptr<function, vec3f>,
    chunk: ptr<function, vec3i>,
    directionSign: vec3f,
    direcctionBound: vec3f,
) {
    let eps = 0.0000001;
    if ((*localPosition).x * directionSign.x >= direcctionBound.x - eps) {
        (*localPosition).x -= directionSign.x;
        (*chunk).x += i32(directionSign.x);
    } else if ((*localPosition).y * directionSign.y >= direcctionBound.y - eps) {
        (*localPosition).y -= directionSign.y;
        (*chunk).y += i32(directionSign.y);
    } else if ((*localPosition).z * directionSign.z >= direcctionBound.z - eps) {
        (*localPosition).z -= directionSign.z;
        (*chunk).z += i32(directionSign.z);
    }
}

fn noise(a: vec2f) -> f32 {
    return perlinNoise3(vec3f(a, 0.0));
}

fn has_star_case(a: vec2i, b: vec2f) -> bool {
    return noise((CLUSTER_SCALE * vec2f(a) + b) % 1.0) > STAR_THRESHOLD;
}

fn has_star(chunk: vec3i) -> bool {
    return has_star_case(chunk.xy + chunk.zx, vec2f(0.724, 0.111))
        && has_star_case(chunk.xz + chunk.zy, vec2f(0.333, 0.777));
}

// bool hasBlackHole(ivec3 chunk) {
//     return rand(0.0001 * vec2(chunk.xy) + 0.002 * vec2(chunk.yz)) > BLACK_HOLE_THRESHOLD;
// }

// vec3 getStarToRayVector(vec3 rayBase, vec3 rayDirection, vec3 starPosition) {
//     float r = (dot(rayDirection, starPosition) - dot(rayDirection, rayBase)) / dot(rayDirection, rayDirection);
//     vec3 pointOnRay = rayBase + r * rayDirection;
//     return pointOnRay - starPosition;
// }

fn getStarPosition(chunk: vec3i, starSize: f32) -> vec3f {
    let position = abs(vec3(rand(vec2f(f32(chunk.x) / f32(chunk.y) + 0.24, f32(chunk.y) / f32(chunk.z) + 0.66)),
                            rand(vec2f(f32(chunk.x) / f32(chunk.z) + 0.73, f32(chunk.z) / f32(chunk.y) + 0.45)),
                            rand(vec2f(f32(chunk.y) / f32(chunk.x) + 0.12, f32(chunk.y) / f32(chunk.z) + 0.76))));

    return starSize * vec3(1.0) + (1.0 - 2.0 * starSize) * position;
}

// vec4 getNebulaColor(vec3 globalPosition, vec3 rayDirection) {
//     vec3 color = vec3(0.0);
//     float spaceLeft = 1.0;

//     const float layerDistance = 10.0;
//     float rayLayerStep = rayDirection.z / layerDistance;

//     const int steps = 4;
//     for (int i = 0; i <= steps; i++) {
//         vec3 noiseeval = globalPosition + rayDirection * ((1.0 - fract(globalPosition.z / layerDistance) + float(i)) * layerDistance / rayDirection.z);
//         noiseeval.xy += noiseeval.z;


//         float value = 0.06 * texture(iChannel0, fract(noiseeval.xy / 60.0)).r;

//         if (i == 0) {
//             value *= 1.0 - fract(globalPosition.z / layerDistance);
//         } else if (i == steps) {
//             value *= fract(globalPosition.z / layerDistance);
//         }

//         float hue = mod(noiseeval.z / layerDistance / 34.444, 1.0);

//         color += spaceLeft * hsv2rgb(vec3(hue, 1.0, value));
//         spaceLeft = max(0.0, spaceLeft - value * 2.0);
//     }
//     return vec4(color, 1.0);
// }

// vec4 getStarGlowColor(float starDistance, float angle, float hue) {
//     float progress = 1.0 - starDistance;
//     return vec4(hsv2rgb(vec3(hue, 0.3, 1.0)), 0.4 * pow(progress, 2.0) * mix(pow(abs(sin(angle * 2.5)), 8.0), 1.0, progress));
// }

// float atan2(vec2 value) {
//     if (value.x > 0.0) {
//         return atan(value.y / value.x);
//     } else if (value.x == 0.0) {
//         return 3.14592 * 0.5 * sign(value.y);
//     } else if (value.y >= 0.0) {
//         return atan(value.y / value.x) + 3.141592;
//     } else {
//         return atan(value.y / value.x) - 3.141592;
//     }
// }

// vec3 getStarColor(vec3 starSurfaceLocation, float seed, float viewDistance) {
//     const float DISTANCE_FAR = 20.0;
//     const float DISTANCE_NEAR = 15.0;

//     if (viewDistance > DISTANCE_FAR) {
//         return vec3(1.0);
//     }

//     float fadeToWhite = max(0.0, (viewDistance - DISTANCE_NEAR) / (DISTANCE_FAR - DISTANCE_NEAR));

//     vec3 coordinate = vec3(acos(starSurfaceLocation.y), atan2(starSurfaceLocation.xz), seed);

//     float progress = pow(texture(iChannel0, fract(0.3 * coordinate.xy + seed * vec2(1.1))).r, 4.0);

//     return mix(mix(vec3(1.0, 0.98, 0.9), vec3(1.0, 0.627, 0.01), progress), vec3(1.0), fadeToWhite);
// }

// vec4 blendColors(vec4 front, vec4 back) {
//       return vec4(mix(back.rgb, front.rgb, front.a / (front.a + back.a)), front.a + back.a - front.a * back.a);
// }

// fn mainImage(frag_coord: vec4f) -> @location(0) vec4f {
fn mainImage(frag_coord: vec4f) -> vec4f {
    let movementDirection = normalize(vec3f(0.01, 0.0, 1.0));

    let rayDirection = getRayDirection(frag_coord.xy, movementDirection);
    let directionSign = sign(rayDirection);
    let directionBound = vec3f(0.5) + 0.5 * directionSign;

    let globalPosition = vec3f(3.14159, 3.14159, 0.0) + (iTime + 1000.0) * FLIGHT_SPEED * movementDirection;
    var chunk = vec3i(globalPosition);
    var localPosition = globalPosition % 1.0;
    moveInsideBox(&localPosition, &chunk, directionSign, directionBound);

    let startChunk = chunk;
    let localStart = localPosition;

    var frag_color = vec4f(0.0);

    for (var i = 0; i < 200; i += 1) {
        movePos(&localPosition, rayDirection, directionBound);
        moveInsideBox(&localPosition, &chunk, directionSign, directionBound);

        if (has_star(chunk)) {
            let starPosition = getStarPosition(chunk, 0.5 * STAR_SIZE);
            let currentDistance = getDistance(chunk - startChunk, localStart, starPosition);
            if (currentDistance > DRAW_DISTANCE && false) {
                break;
            }

    //         // This vector points from the center of the star to the closest point on the ray (orthogonal to the ray)
    //         vec3 starToRayVector = getStarToRayVector(localPosition, rayDirection, starPosition);
    //         // Distance between ray and star
    //         float distanceToStar = length(starToRayVector);
    //         distanceToStar *= 2.0;

    //         if (distanceToStar < STAR_SIZE) {
    //             float starMaxBrightness = clamp((DRAW_DISTANCE - currentDistance) / FADEOUT_DISTANCE, 0.001, 1.0);

    //             float starColorSeed = (float(chunk.x) + 13.0 * float(chunk.y) + 7.0 * float(chunk.z)) * 0.00453;
    //             if (distanceToStar < STAR_SIZE * STAR_CORE_SIZE) {
    //                 // This vector points from the center of the star to the point of the star sphere surface that this ray hits
    //                 vec3 starSurfaceVector = normalize(starToRayVector + rayDirection * sqrt(pow(STAR_CORE_SIZE * STAR_SIZE, 2.0) - pow(distanceToStar, 2.0)));

    //                 frag_color = blendColors(frag_color, vec4(getStarColor(starSurfaceVector, starColorSeed, currentDistance), starMaxBrightness));
    //                 break;
    //             } else {
    //                 float localStarDistance = ((distanceToStar / STAR_SIZE) - STAR_CORE_SIZE) / (1.0 - STAR_CORE_SIZE);
    //                 vec4 glowColor = getStarGlowColor(localStarDistance, atan2(starToRayVector.xy), starColorSeed);
    //                 glowColor.a *= starMaxBrightness;
    //                 frag_color = blendColors(frag_color, glowColor);
    //             }
    //         }
        }
    //     } else if (hasBlackHole(chunk)) {
    //         const vec3 blackHolePosition = vec3(0.5);
    //         float currentDistance = getDistance(chunk - startChunk, localStart, blackHolePosition);
    //         float fadeout = min(1.0, (DRAW_DISTANCE - currentDistance) / FADEOUT_DISTANCE);

    //         // This vector points from the center of the black hole to the closest point on the ray (orthogonal to the ray)
    //         vec3 coreToRayVector = getStarToRayVector(localPosition, rayDirection, blackHolePosition);
    //         float distanceToCore = length(coreToRayVector);
    //         if (distanceToCore < BLACK_HOLE_CORE_RADIUS * 0.5) {
    //             frag_color = blendColors(frag_color, vec4(vec3(0.0), fadeout));
    //             break;
    //         } else if (distanceToCore < 0.5) {
    //             rayDirection = normalize(rayDirection - fadeout * (BLACK_HOLE_DISTORTION / distanceToCore - BLACK_HOLE_DISTORTION / 0.5) * coreToRayVector / distanceToCore);
    //         }
    //     }

    //     if (length(vec3(chunk - startChunk)) > DRAW_DISTANCE) {
    //         break;
    //     }
    }

    if (frag_color.a < 1.0) {
        // frag_color = blendColors(frag_color, getNebulaColor(globalPosition, rayDirection));
    }
    return frag_color;
}
