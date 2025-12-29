// Vertex Shader
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    
    // Posiciones del triángulo
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, -0.5),
        vec2<f32>(0.5, 0.5),
        vec2<f32>(-0.5, 0.5)
    );
    
    // Colores del triángulo
    var colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0),  // Rojo
        vec3<f32>(0.0, 1.0, 0.0),  // Verde
        vec3<f32>(0.0, 0.0, 1.0)   // Azul
    );
    
    out.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    out.color = colors[vertex_index];
    
    return out;
}

// Fragment Shader
struct Uniforms {
    time: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Animar los colores usando funciones trigonométricas
    var animated_color = in.color;
    
    let t = uniforms.time;
    animated_color.r *= abs(sin(t * 1.0));
    animated_color.g *= abs(sin(t * 1.3 + 2.0));
    animated_color.b *= abs(sin(t * 0.8 + 4.0));
    
    // Añadir un brillo pulsante
    let brightness = 0.5 + 0.5 * sin(t * 2.0);
    animated_color = mix(in.color * 0.3, animated_color, brightness);
    
    return vec4<f32>(animated_color, 1.0);
}
