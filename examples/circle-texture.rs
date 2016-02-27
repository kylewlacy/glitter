extern crate sdl2;
#[macro_use] extern crate glitter;

use sdl2::video::GLProfile;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use glitter::prelude::*;

fn circle_image(width: usize, height: usize, radius: f32) -> glitter::Pixels {
    let (center_x, center_y) = (width as f32/2.0, height as f32/2.0);

    let mut pixels = glitter::Pixels::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let dx = center_x - x as f32;
            let dy = center_y - y as f32;
            let distance = (dx*dx + dy*dy).sqrt();

            let color = if distance < radius {
                // The point is within the circle, so it should be red
                glitter::Pixel::rgb(0xFF0000)
            }
            else {
                // The point is outside the circle, so it should be black
                glitter::Pixel::rgb(0x000000)
            };
            pixels[y][x] = color;
        }
    }

    pixels
}

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
    let window = video.window("Hello Circle!", 800, 600)
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
    // (a 2D position coordinate, and a texture coordinate)
    #[derive(Clone, Copy)]
    struct Vertex {
        position: [f32; 2],
        tex_coord: [f32; 2]
    }

    // Mark our `Vertex` as a type that we can treat as a vertex for our shader
    impl_vertex_data!(Vertex, position, tex_coord);

    // The vertices the make up our screen's quad
    let vertices = [
        Vertex { position: [-1.0, -1.0], tex_coord: [0.0, 0.0] },
        Vertex { position: [-1.0,  1.0], tex_coord: [0.0, 1.0] },
        Vertex { position: [ 1.0,  1.0], tex_coord: [1.0, 1.0] },
        Vertex { position: [ 1.0, -1.0], tex_coord: [1.0, 0.0] }
    ];

    // The indices that make up our screen's quad
    let indices = [
        0, 1, 2,
        0, 2, 3
    ];

    // Create our circle texture.
    let mut circle = gl.build_texture_2d()
                       .image_2d(&circle_image(800, 600, 250.0))
                       .min_filter(glitter::NEAREST)
                       .mag_filter(glitter::NEAREST)
                       .wrap_s(glitter::CLAMP_TO_EDGE)
                       .wrap_t(glitter::CLAMP_TO_EDGE)
                       .unwrap();

    // The vertex shader, which translates our vertices to screen coordinates
    let vertex_source = r##"#version 100
        // Our inputs (the fields from our `Vertex` struct)
        attribute vec2 position;
        attribute vec2 texCoord;

        // Our output (the texture coordinate for our fragment shader)
        varying vec2 _texCoord;

        void main() {
            gl_Position = vec4(position, -1.0, 1.0);
            _texCoord = texCoord;
        }
    "##;

    // The fragment shader, which knows how to color pixels for the final image
    let fragment_source = r##"#version 100
        // Our uniform (the texture we read from)
        uniform sampler2D sampler;

        // Our input (the texture coordinate copied from our vertex shader)
        varying highp vec2 _texCoord;

        void main() {
            gl_FragColor = texture2D(sampler, _texCoord);
        }
    "##;

    // Compile our vertex and fragment shader, panicking if there was a
    // compilation error
    let vertex_shader = gl.build_vertex_shader(vertex_source).unwrap();
    let fragment_shader = gl.build_fragment_shader(fragment_source).unwrap();

    // Combine our shaders into a program, panicking if there was a
    // linking error
    let mut program = gl.build_program(&[vertex_shader, fragment_shader]).unwrap();

    // Create a buffer to send our quad's vertices to
    let mut vertex_buffer: glitter::VertexBuffer<Vertex> = gl.new_vertex_buffer();

    // Create a buffer to send our quad's indices to
    let mut index_buffer: glitter::IndexBuffer<u16> = gl.new_index_buffer();

    // The "attrib pointers" that connects the input attributes from our
    // vertex shader to the fields of our `Vertex` struct
    let attribs = attrib_pointers! {
        position => gl.get_attrib_location(&program, "position").unwrap(),
        tex_coord => gl.get_attrib_location(&program, "texCoord").unwrap()
    };

    // Add our attributes to our vertex buffer
    vertex_buffer.bind_attrib_pointers(attribs);

    // Bind the vertex buffer to the OpenGL context, so that we can actually
    // send our vertex data to it
    let (mut gl_vertex_buffer, gl) = gl.bind_vertex_buffer(&mut vertex_buffer);

    // Send our vertex data to our binding. We use `glitter::STATIC_DRAW`
    // because the geometry of our triangle is static
    gl.buffer_vertices(&mut gl_vertex_buffer, &vertices, glitter::STATIC_DRAW);

    // Bind the index buffer to the OpenGL context, so that we can actually
    // send our index data to it
    let (mut gl_index_buffer, gl) = gl.bind_index_buffer(&mut index_buffer);

    // Send our index data to our binding.
    gl.buffer_indices(&mut gl_index_buffer, &indices, glitter::STATIC_DRAW);

    // Set texture unit 0 as the active texture unit
    let (gl_tex_unit, gl) = gl.active_texture_0();

    // Bind our circle texture to the texture unit
    let (_, gl_tex_unit) = gl_tex_unit.bind_texture_2d(&mut circle);

    // Get the sampler of the texture unit
    let circle_sampler = gl_tex_unit.sampler();

    // Get the program uniform that we set the sampler to, panicking
    // if the sampler was not found
    let sampler_uniform = gl.get_uniform_location(&program, "sampler").unwrap();

    // Bind our program to the OpenGL context
    let (gl_program, gl) = gl.use_program(&mut program);

    // Set the sampler uniform to reference our circle texture
    gl.set_uniform(&gl_program, sampler_uniform, circle_sampler);

    // Finally, draw the screen quad!
    gl.draw_elements_buffered_vbo(&gl_vertex_buffer,
                                  &gl_index_buffer,
                                  glitter::TRIANGLES);

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
