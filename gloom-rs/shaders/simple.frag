#version 430 core

in layout(location=1)  vec3 vertexColour;
 
out vec4 color;

void main()
{
    color = vec4(vertexColour, 1.0f);
}