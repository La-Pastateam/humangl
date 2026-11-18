#version 450

layout(push_constant) uniform PushConstants {
	layout(offset = 64) uint color;
} pcs;

layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec2 fragTexCoord;

layout(location = 0) out vec4 outColor;

void main() {
	outColor = unpackUnorm4x8(pcs.color);
}
