#version 450
layout(location = 0) in vec2 Frag_UV;
layout(location = 1) in vec4 Frag_Color;

layout(location = 0) out vec4 Out_Color;

layout(set = 0, binding = 0) uniform sampler2D sTexture;

void main() {
    Out_Color = Frag_Color * texture(sTexture, Frag_UV);
}
