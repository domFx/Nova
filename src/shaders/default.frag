#version 450

layout(location = 0) in vec4 fragColor;
layout(location = 1) in vec2 fragTextCoord;
layout(location = 2) in flat int fragTextIndex;

layout(location = 0) out vec4 outColor;

layout(set = 0, binding = 1) uniform sampler2D texSampler[8];

void main() {
    if (fragTextIndex >= 0) {
        // vec4 sampled = vec4(1.0, 1.0, 1.0, texture(texSampler[fragTextIndex], fragTextCoord).r);
        // outColor = fragColor * sampled;

        outColor = texture(texSampler[fragTextIndex], fragTextCoord);
    } else {
        outColor = fragColor;
    }
}