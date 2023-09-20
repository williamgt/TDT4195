#version 430 core

in layout(location=0) vec3 position;
in layout(location=1) vec4 color;

out layout(location=1) vec4 outColor;

uniform layout(location=0) mat4x4 m;

void main()
{
    outColor = color;

    vec4 tempPos = vec4(position, 1.0f);

    gl_Position = tempPos * m;
}