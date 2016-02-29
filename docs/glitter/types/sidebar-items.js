initSidebarItems({"constant":[["BLEND","Blend newly-computed fragment colors with the current values in the color buffer."],["BYTE","A signed 8-bit byte."],["COLOR_BUFFER_BIT","The color buffer, which stores color information for each fragment (or pixel)."],["CULL_FACE","Cull polygons, based on their winding in window coordinates."],["DEPTH_BUFFER_BIT","The depth buffer, which stores distance information for each fragment when depth testing is enabled."],["DEPTH_TEST","Perform a depth test for each fragment, only drawing fragments that are not obscured by other geometry. Also updates the depth buffer appropriately."],["DITHER","Dither color components or indices."],["FIXED","A signed 32-bit, fixed-point number in 16.16 form."],["FLOAT","A 32-bit, IEEE floating-point number."],["LINES","Draw each pair of vertices as individual line segments."],["LINE_LOOP","Draw a self-connected line segment, where each vertex is connected to the next, and the last vertex connects to the first."],["LINE_STRIP","Draw a connected line segment, where each vertex is connected to the next. The first and last vertex are treated as the start and end points."],["POINTS","Draw each vertex as a single point."],["POLYGON_OFFSET_FILL","When filling a polygon, add an offset to each fragment's depth value."],["SAMPLE_ALPHA_TO_COVERAGE","When multisampling, use the alpha value from the sample location."],["SAMPLE_COVERAGE","When multisampling, use the preset sample coverage value as the alpha value."],["SCISSCOR_TEST","Only draw fragments within the scissor rectangle."],["SHORT","A signed 16-bit short."],["STENCIL_BUFFER_BIT","The stencil buffer, which stores information about which fragments should be kept or discarded when stencil testing is enabled."],["STENCIL_TEST","Perform a stencil test for each fragment, only drawing fragments that pass the currently-set stencil operation. Also updates the stencil buffer appropriately."],["TRIANGLES","Draw each group of three vertices as a triangle."],["TRIANGLE_FAN","Draw the vertices as a triangle fan. The first vertex, v1 is the fan's 'center'. Vertices v2 and v3 form the first triangle with the center, v1. Then vertices v3, v4, and v1 form the next triangle, then vertices v4, v5, and v1, and so on."],["TRIANGLE_STRIP","Draw the vertices as a strip of triangles. The first three vertices form the first triangle, then the next vertex plus the previous two vertices form the next triangle, and so on. For example, vertices v1, v2, and v3 form the first triangle, then vertices v2, v3, and v4 form the next, and so on."],["UNSIGNED_BYTE","An unsigned 8-bit byte."],["UNSIGNED_SHORT","An unsigned 16-bit short."]],"enum":[["Capability","The OpenGL drawing capabilities that can be enabled or disabled."],["DataType","The different OpenGL data types."],["DrawingMode","The primitive drawing modes for drawing raw vertex data."],["GLError","The various possible OpenGL errors."],["GLFramebufferError","The possible framebuffer-incomplete errors."]],"struct":[["BufferBits","The possible buffers that the active framebuffer may contain."],["Color","A color, with floating-point RGBA components."],["Viewport","An OpenGL viewport, with an origin and size, with integer components."]],"trait":[["GLObject","An OpenGL object."]]});