#version 430 core

in layout(location=0) vec3 position;
in layout(location=1) vec4 color;

out layout(location=1) vec4 outColor;

void main()
{
    outColor = color;
    gl_Position = vec4(position, 1.0f);
}