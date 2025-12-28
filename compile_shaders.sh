#!/bin/bash
# Script para compilar shaders GLSL a SPIR-V

echo "Compilando shaders..."

# Compilar vertex shader
glslc shaders/triangle.vert -o shaders/triangle.vert.spv
if [ $? -eq 0 ]; then
    echo "✓ triangle.vert.spv compilado"
else
    echo "✗ Error compilando triangle.vert"
    exit 1
fi

# Compilar fragment shader
glslc shaders/triangle.frag -o shaders/triangle.frag.spv
if [ $? -eq 0 ]; then
    echo "✓ triangle.frag.spv compilado"
else
    echo "✗ Error compilando triangle.frag"
    exit 1
fi

echo "✓ Todos los shaders compilados exitosamente"
