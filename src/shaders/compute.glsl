#version 450

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(rgba32f, binding = 0) uniform image2D img_output;

void main() {
    vec2 dims = vec2(imageSize(img_output));
    ivec3 pixel_coords = ivec3(gl_GlobalInvocationID.xyz);

    vec2 uv = pixel_coords.xy / dims;
    imageStore(img_output, pixel_coords.xy, vec4(uv, 0.0, 1.0));
}
