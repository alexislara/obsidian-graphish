#version 450

layout(location = 0) in vec3 fragColor;

layout(location = 0) out vec4 outColor;

// Push constant para el tiempo
layout(push_constant) uniform PushConstants {
    float time;
} pc;

void main() {
    // Animar los colores usando funciones trigonométricas
    vec3 animatedColor = fragColor;
    
    // Rotar los colores RGB cíclicamente con el tiempo
    float t = pc.time;
    animatedColor.r *= abs(sin(t * 1.0));
    animatedColor.g *= abs(sin(t * 1.3 + 2.0));
    animatedColor.b *= abs(sin(t * 0.8 + 4.0));
    
    // Añadir un brillo pulsante general
    float brightness = 0.5 + 0.5 * sin(t * 2.0);
    animatedColor = mix(fragColor * 0.3, animatedColor, brightness);
    
    outColor = vec4(animatedColor, 1.0);
}
