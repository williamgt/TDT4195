#version 430 core

in layout(location=1)  vec3 vertexColour;
//uniform layout(location = 1) vec2 initialScreenHeightWidth;
 
out vec4 color;

void main()
{
    vec3 newColor = vec3(0.0f, vertexColour.y, vertexColour.z);
    if(newColor.y <= 0 && newColor.z <= 0) {
        newColor.x = 1.0f;
    } 
    color = vec4(newColor, 1.0f);

    //Checker color
    double divisor = 10.0f; //Size of checker square
    int checkerX = int(floor(gl_FragCoord.x / divisor));
    int checkerY = int(floor(gl_FragCoord.y / divisor));
    int checker = (checkerX + checkerY) % 2;

    if (checker == 0) { //Black checker square if in specified area
        color = vec4(newColor, 1.0f);
    } else {
        color = vec4(0.0f, 0.0f, 0.0f, 1.0f);
    }
}