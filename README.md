# Obsidian Graphish - Motor Gráfico Vulkan

Motor gráfico 3D escrito en Rust utilizando la API de Vulkan.

## ✨ Características

- ✅ Renderizado con Vulkan 1.3
- ✅ Ventanas multiplataforma (X11, Wayland) con winit
- ✅ Pipeline gráfico completo
- ✅ Sistema de swapchain con sincronización
- ✅ Shaders GLSL compilados a SPIR-V
- ✅ Arquitectura modular y extensible

## 🏗️ Estructura del Proyecto

```
obsidian-graphish/
├── src/
│   ├── main.rs              # Punto de entrada, loop de eventos
│   ├── core/                # Componentes principales del motor
│   │   ├── mod.rs
│   │   ├── engine.rs        # Motor principal (orquestador)
│   │   ├── instance.rs      # VkInstance (inicialización Vulkan)
│   │   └── window.rs        # VkSurface (integración ventana-Vulkan)
│   └── renderer/            # Sistema de renderizado
│       ├── mod.rs
│       ├── device.rs        # VkDevice (GPU lógica y física)
│       ├── swapchain.rs     # VkSwapchain (cadena de imágenes)
│       └── pipeline.rs      # VkPipeline (pipeline gráfico)
├── shaders/                 # Shaders GLSL
│   ├── triangle.vert        # Vertex shader (triángulo)
│   ├── triangle.frag        # Fragment shader (colores)
│   ├── triangle.vert.spv    # Compilado SPIR-V
│   └── triangle.frag.spv    # Compilado SPIR-V
├── Cargo.toml
└── compile_shaders.sh       # Script para compilar shaders
```

## 🔧 Dependencias

```toml
ash = "0.38.0"           # Bindings de Vulkan
ash-window = "0.13.0"    # Conexión ventana-Vulkan
winit = "0.30.12"        # Manejo de ventanas
raw-window-handle = "0.6.2"  # Handles de ventana
glam = "0.30.9"          # Matemáticas para gráficos
```

## 📋 Requisitos del Sistema

- **Rust**: 1.75+ (edition 2024)
- **Vulkan**: 1.3+
- **GPU**: Compatible con Vulkan
- **OS**: Linux (X11/Wayland), Windows, macOS

### Herramientas de Compilación

```bash
# Ubuntu/Debian
sudo apt install glslang-tools vulkan-tools

# Arch Linux
sudo pacman -S glslang vulkan-tools

# Verificar instalación
glslangValidator --version
vulkaninfo --summary
```

## 🚀 Compilación y Ejecución

### 1. Compilar Shaders

```bash
./compile_shaders.sh
# O manualmente:
glslangValidator -V shaders/triangle.vert -o shaders/triangle.vert.spv
glslangValidator -V shaders/triangle.frag -o shaders/triangle.frag.spv
```

### 2. Compilar el Proyecto

```bash
cargo build --release
```

### 3. Ejecutar

```bash
cargo run --release
```

## 🎨 Arquitectura

### Flujo de Inicialización

```
1. Window (winit)
   ↓
2. VkInstance (ash::Entry)
   ↓
3. VkSurface (ash_window)
   ↓
4. VkDevice (Physical + Logical)
   ↓
5. VkSwapchain (Cadena de imágenes)
   ↓
6. VkPipeline (Render pass + Shaders)
   ↓
7. Command Pool + Buffers
   ↓
8. Sync Objects (Semaphores, Fences)
```

### Ciclo de Renderizado

```rust
loop {
    1. wait_for_fences()          // Esperar frame anterior
    2. acquire_next_image()       // Obtener imagen del swapchain
    3. reset_command_buffer()     // Limpiar buffer de comandos
    4. record_command_buffer()    // Grabar comandos de dibujo
       ├─ begin_render_pass()
       ├─ bind_pipeline()
       ├─ cmd_draw()
       └─ end_render_pass()
    5. queue_submit()             // Enviar a GPU
    6. queue_present()            // Presentar en pantalla
}
```

## 🧩 Componentes Principales

### Engine (`core/engine.rs`)

Motor principal que coordina todos los subsistemas:
- Inicialización completa de Vulkan
- Loop de renderizado
- Gestión de sincronización
- Limpieza ordenada de recursos

### VkInstance (`core/instance.rs`)

Capa de abstracción para la instancia de Vulkan:
- Carga de la librería Vulkan
- Activación de capas de validación (debug)
- Enumeración de dispositivos físicos

### VkSurface (`core/window.rs`)

Conexión entre la ventana y Vulkan:
- Creación de superficie para renderizado
- Consultas de capacidades de superficie
- Soporte multiplataforma (X11, Wayland, Windows, etc.)

### VkDevice (`renderer/device.rs`)

Gestión del dispositivo gráfico:
- Selección de GPU más adecuada
- Creación de dispositivo lógico
- Familias de colas (graphics, present)

### VkSwapchain (`renderer/swapchain.rs`)

Cadena de imágenes para presentación:
- Configuración de formato y modo de presentación
- Creación de image views
- Gestión de múltiples imágenes en vuelo

### VkPipeline (`renderer/pipeline.rs`)

Pipeline gráfico completo:
- Render pass
- Shaders (vertex + fragment)
- Estados de pipeline (rasterización, viewport, etc.)
- Framebuffers

## 🎯 Próximas Características

- [ ] Sistema de cámara 3D
- [ ] Carga de modelos (OBJ, glTF)
- [ ] Sistema de materiales
- [ ] Iluminación (Phong, PBR)
- [ ] Texturas
- [ ] Sistema de partículas
- [ ] Post-procesado
- [ ] Shadow mapping
- [ ] Occlusion culling

## 🐛 Troubleshooting

### Error: "No Vulkan drivers found"

```bash
# Verificar drivers
vulkaninfo

# Instalar drivers (NVIDIA)
sudo apt install nvidia-driver-XXX vulkan-tools

# Instalar drivers (AMD)
sudo apt install mesa-vulkan-drivers vulkan-tools
```

### Error: "Shaders no compilados"

```bash
# Compilar shaders manualmente
cd shaders
glslangValidator -V triangle.vert -o triangle.vert.spv
glslangValidator -V triangle.frag -o triangle.frag.spv
```

### Advertencias de Validación

Las capas de validación están activas en modo debug:
```bash
# Ejecutar sin validación
VK_INSTANCE_LAYERS= cargo run --release
```

## 📚 Recursos

- [Vulkan Tutorial](https://vulkan-tutorial.com/)
- [Ash Documentation](https://docs.rs/ash/)
- [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/)
- [Learn Vulkan](https://vkguide.dev/)

## 📄 Licencia

Este proyecto es de código abierto y está disponible bajo la licencia MIT.

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Por favor, abre un issue o pull request.

---

**Desarrollado con ❤️ en Rust**
