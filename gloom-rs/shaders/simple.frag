#version 430 core

in layout(location=1)  vec4 vertexColour;
in layout(location=2)  vec3 normal;
 
out vec4 color;

void main()
{
    vec3 lightDirection = normalize(vec3(0.8f, -0.5f, 0.6f));
    float colorContrib = max(0.0f, dot(normal, (-lightDirection)));
    vec4 vertexColorWithContrib = vertexColour * colorContrib; 
    color = vec4(vertexColorWithContrib.xyz, vertexColour.a);
}