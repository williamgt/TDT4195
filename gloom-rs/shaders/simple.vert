#version 430 core

in layout(location=0) vec3 position;
in layout(location=1) vec4 color;

out layout(location=1) vec4 outColor;

uniform layout(location=0) float oscillate;

void main()
{
    outColor = color;

    vec4 tempPos = vec4(position, 1.0f);
    mat4x4 m = {
        {oscillate, 0.0f, 0.0f, 0.0f}, //col1
        {0.0f, 1.0f, 0.0f, 0.0f}, //col2
        {0.0f, 0.0f, 1.0f, 0.0f}, //col3
        {0.0f, 0.0f, 0.0f, 1.0f}, //col4
    };

    mat4x4 identity = mat4(1.0f); //Initialises the identity matrix

    gl_Position = tempPos * m;
}