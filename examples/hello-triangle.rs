extern crate sdl2;
#[macro_use] extern crate glitter;

use sdl2::video::GLProfile;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use glitter::prelude::*;

fn setup_gl(video: &sdl2::VideoSubsystem) {
    let gl_attr = video.gl_attr();

    // Use OpenGL 4.1 core. Note that glitter is (currently) only designed
    // for OpenGL ES 2.0, but OpenGL 4.1 added the GL_ARB_ES2_compatibility
    // extension, which adds OpenGL ES 2 compatibility
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    gl_attr.set_context_flags().debug().set();

    // Load the system's OpenGL library
    video.gl_load_library_default().expect("Failed to load OpenGL library");

    // Load OpenGL function pointers
    unsafe {
        glitter::Context::load_with(|s| {
            video.gl_get_proc_address(s) as *const _
        });
    }
}

unsafe fn gl_vao_hack() {
    use glitter::gl;
    use glitter::gl::types::GLuint;

    // So... OpenGL 4.1 and OpenGL ES 2.0 aren't EXACTLY compatible.
    // For example, look at glEnableVertexAttribArray. In OpenGL 4.1, it
    // requires a vertex array object to be currently bound. However,
    // OpenGL ES 2.0 doesn't have vertex array objects (without an extension).
    // To work around this, we just create and bind a vertex array
    // object globally, so we can use these functions as we would in
    // OpenGL ES 2.0. This specific issue will be solved in a future release
    // of glitter.
    let mut vertex_array_object: GLuint = 0;
    gl::GenVertexArrays(1, &mut vertex_array_object);
    gl::BindVertexArray(vertex_array_object);
}

fn main() {
    // Initialize SDL and the video submodule
    let sdl = sdl2::init().expect("Failed to initailize SDL");
    let video = sdl.video().expect("Failed to intialize SDL video system");

    // Do all the necessary SDL OpenGL setup
    setup_gl(&video);

    // Create our window (and make it usable with OpenGL)
    let window = video.window("Hello Triangle!", 800, 600)
                      .opengl()
                      .build()
                      .expect("Failed to create SDL window");

    // Create a new OpenGL context
    let _context = window.gl_create_context().expect("Failed to create OpenGL context");

    // Bind the window's OpenGL context
    window.gl_set_context_to_current().expect("Failed to set current context");

    // Workaround for OpenGL 4.1/OpenGL ES 2 vertex array object disparity
    unsafe { gl_vao_hack(); }

    // Get the current OpenGL context
    let mut gl = unsafe { glitter::Context::current_context() };

    // Clear the screen to black
    gl.clear_color(glitter::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 });
    gl.clear(glitter::COLOR_BUFFER_BIT);

    // The data that makes up a single vertex of our triangle
    // (a 2D position coordinate, and an RGB color value)
    #[derive(Clone, Copy)]
    struct Vertex {
        position: [f32; 2],
        color: [f32; 3]
    }

    // Mark our `Vertex` as a type that we can treat as a vertex for our shader
    impl_vertex_data!(Vertex, position, color);

    // The vertices the make up our triangle
    let vertices = [
        Vertex { position: [-1.0, -1.0], color: [1.0, 0.0, 0.0] },
        Vertex { position: [ 0.0,  1.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [ 1.0, -1.0], color: [0.0, 0.0, 1.0] }
    ];

    // The vertex shader, which translates our vertices to screen coordinates
    let vertex_source = r##"#version 100
        // Our inputs (the fields from our `Vertex` struct)
        attribute vec2 position;
        attribute vec3 color;

        // Our output (the color for our fragment shader)
        varying vec3 _color;

        void main() {
            gl_Position = vec4(position, -1.0, 1.0);
            _color = color;
        }
    "##;

    // The fragment shader, which knows how to color pixels for the final image
    let fragment_source = r##"#version 100
        // Our input (the color copied from our vertex shader)
        varying highp vec3 _color;

        void main() {
            gl_FragColor = vec4(_color, 1.0);
        }
    "##;

    // Compile our vertex and fragment shader, panicking if there was a
    // compilation error
    let vertex_shader = gl.build_vertex_shader(vertex_source).unwrap();
    let fragment_shader = gl.build_fragment_shader(fragment_source).unwrap();

    // Combine our shaders into a program, panicking if there was a
    // linking error
    let mut program = gl.build_program(&[vertex_shader, fragment_shader]).unwrap();

    // Create a buffer to send our triangle's vertices to
    let mut vertex_buffer: glitter::VertexBuffer<Vertex> = gl.new_vertex_buffer();

    // The "attrib pointers" that connects the input attributes from our
    // vertex shader to the fields of our `Vertex` struct
    let attribs = attrib_pointers! {
        position => gl.get_attrib_location(&program, "position").unwrap(),
        color => gl.get_attrib_location(&program, "color").unwrap()
    };

    // Add our attributes to our vertex buffer
    vertex_buffer.bind_attrib_pointers(attribs);

    // Bind the vertex buffer to the OpenGL context, so that we can actually
    // send our vertex data to it
    let (mut gl_vertex_buffer, gl) = gl.bind_vertex_buffer(&mut vertex_buffer);

    // Send our vertex data to our binding. We use `glitter::STATIC_DRAW`
    // because the geometry of our triangle is static
    gl.buffer_vertices(&mut gl_vertex_buffer, &vertices, glitter::STATIC_DRAW);

    // Bind our program to the OpenGL context
    let (_, gl) = gl.use_program(&mut program);

    // Finally, draw the points from our vertex buffer!
    gl.draw_arrays_vbo(&gl_vertex_buffer, glitter::TRIANGLES);

    // Display what we've rendered so far
    window.gl_swap_window();

    // Handle any extra input events
    let mut event_pump = sdl.event_pump().expect("Failed to get SDL events");
    'running: loop {
        // Handle any input events we need to
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => { }
            }
        }

        // Our main loop goes here (in most applications, this is
        // where we would actually do our rendering)
    }
}
