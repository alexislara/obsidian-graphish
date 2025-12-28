#version 450

// Input del vertex shader
layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec2 fragTexCoord;

// Output: color final del pixel
layout(location = 0) out vec4 outColor;

// Push constant para efectos (tiempo + color del sprite)
// Alineado a vec4 (16 bytes) para Vulkan
layout(push_constant) uniform PushConstants {
    vec4 data;  // x=time, y=colorR, z=colorG, w=colorB
} pc;

void main() {
    float time = pc.data.x;
    vec3 spriteColor = pc.data.yzw;
    
    // Combinar color del vértice con el color del sprite
    vec3 color = fragColor * spriteColor;
    
    // Efecto pulsante muy sutil basado en el tiempo
    float pulse = 0.9 + 0.1 * sin(time * 2.0);
    color *= pulse;
    
    outColor = vec4(color, 1.0);
}
