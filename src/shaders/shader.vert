#version 450

layout(binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec2 inPosition;
layout(location = 1) in vec4 inColor;
layout(location = 2) in vec2 inTextCoord;
layout(location = 3) in int inTextIndex;

layout(location = 0) out vec4 fragColor;
layout(location = 1) out vec2 fragTextCoord;
layout(location = 2) out int fragTextIndex;

void main() {
    // MVP Projection
    gl_Position = ubo.proj * ubo.view * ubo.model * vec4(inPosition, 0.0, 1.0);
    //    gl_Position = vec4(inPosition, 0.0, 1.0);

    fragColor = inColor;
    fragTextCoord = inTextCoord;
    fragTextIndex = inTextIndex;
}
