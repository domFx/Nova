#version 450

layout(location = 0) in vec4 fragColor;
layout(location = 1) in vec2 fragTextCoord;

layout(location = 0) out vec4 outColor;

layout(binding = 1) uniform sampler2D texSampler;

void main() {
    outColor = fragColor;
    // outColor = texture(texSampler, fragTextCoord);
}
