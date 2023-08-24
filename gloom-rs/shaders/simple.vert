#version 430 core

in layout(location=0) vec3 position;
in layout(location=1) vec3 color;

out layout(location=1) vec3 outColor;

void main()
{
    outColor = color;
    vec3 flipped = vec3(-position.x, -position.y, -position.z); 
    gl_Position = vec4(flipped, 1.0f);
}