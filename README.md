# glitter - A safe, low-level, zero-cost Rust OpenGL wrapper library

glitter is an experimental Rust library designed to wrap the OpenGL graphics
API. It's designed for applications where performance and correctness are
critical, such as in game and game engine development. **Currently, glitter
only supports the OpenGL ES2 API**, but this will change in an upcoming
release.

# [Documentation](https://kylewlacy.github.io/glitter/docs/glitter/index.html)

(The documentation, like the rest of the project, is still very much
work-in-progress. Every public API has been documented, but there's still
a lot to clean up!)

# Show me the code!

There are also currently two examples in the [`examples`](https://github.com/kylewlacy/tree/master/examples)
directory, which use the [`sdl2`](https://crates.io/crates/sdl2) crate.
These can be run using `cargo`, such as with the following:

```sh
$ cargo run --example hello-triangle
```

Also, just for a taste, here's a snippet for a simple "hello triangle"
example:

```rust
#[macro_use] extern crate glitter;
use glitter::prelude::*;

// ...platform-specific OpenGL setup...
let gl = unsafe { glitter::Context::current_context() };

gl.clear_color(glitter::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 });
gl.clear(glitter::COLOR_BUFFER_BIT);

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3]
}

impl_vertex_data!(Vertex, position, color);

let vertices = [
    Vertex { position: [-1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [ 0.0,  1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 1.0, -1.0], color: [0.0, 0.0, 1.0] }
];

let vertex_source = r##"#version 100
    attribute vec2 position;
    attribute vec3 color;

    varying vec3 _color;

    void main() {
        gl_Position = vec4(position, -1.0, -1.0);
        _color = color;
    }
"##;

let fragment_source = r##"#version 100
    varying highp vec3 _color;

    void main() {
        gl_FragColor = vec4(_color, 1.0);
    }
"##;

let vertex_shader = gl.build_vertex_shader(vertex_source).unwrap();
let fragment_shader = gl.build_fragment_shader(fragment_source).unwrap();

let mut program = gl.build_program(&[vertex_shader, fragment_shader]).unwrap();

let mut vertex_buffer: glitter::VertexBuffer<Vertex> = gl.new_vertex_buffer();

let attribs = attrib_pointers! {
    position => gl.get_attrib_location(&program "position").unwrap(),
    color => gl.get_attrib_location(&program, "color").unwrap()
};
vertex_buffer.bind_attrib_pointers(attribs);

let (mut gl_vertex_buffer, gl) = gl.bind_vertex_buffer(&mut vertex_buffer);
gl.buffer_vertices(&mut gl_vertex_buffer, &vertices, glitter::STATIC_DRAW);

let (_, gl) = gl.use_program(&mut program);

gl.draw_arrays_vbo(&gl_vertex_buffer, glitter::TRIANGLES);
```

# Design

## Safety
glitter is designed to statically prevent OpenGL errors where possible, using
compile-time checks. Additionally, in debug mode, glitter checks for all OpenGL
errors by default.

## Low Level
Most OpenGL functions have a 1:1 parallel in glitter. Additionally,
a small set of higher-level abstractions are provided where applicable,
such as with the [`VertexBuffer`]
(https://kylewlacy.github.io/glitter/docs/glitter/vertex_buffer/struct.VertexBuffer.html)
type, which provides a low-overhead, high-level interface for creating,
buffering, and drawing vertex data.

## Zero Cost
Many of the core types in glitter are composed of [zero-sized types]
(https://doc.rust-lang.org/nomicon/vec-zsts.html) or [pointers to zero-sized
types](http://www.wabbo.org/blog/2014/03aug_09aug.html) (such as the
[`Context`](https://kylewlacy.github.io/glitter/docs/glitter/context/type.Context.html)
type). This means that glitter method calls can often compile directly OpenGL
function invocations.

----------

For more details about how glitter achieves its design goals, consult
the [documentation](https://kylewlacy.github.io/glitter/docs/glitter/index.html).

# What's next?
glitter is very much still in the 'experimental' phase. Functions will be
buggy, API calls will break, things will be outright missing. So, here's
an off-the-cuff list of things coming in the near(ish) future:

- [ ] A testing setup, and a test suite
- [ ] Add missing OpenGL ES 2.0 methods
- [ ] Fix the ugly "VAO hack" in the examples
- [ ] Set up a framework for targeting different OpenGL versions (currently
OpenGL ES 2.0 is baked in- the goal is to be able to list which OpenGL
versions and extensions an application wants to target, and only code
for that set of features gets generated)
- [ ] Clean up the documentation (make docs clearer, add more examples, remove
dead links, etc.)

# FAQ

*Should I use this?*

> No. Not unless you like using broken, buggy, untested libraries with API's
> that breaks between minor releases. That is, until glitter reaches 1.0 :)

*I'm okay with broken, buggy, untested libraries with API's that break
between minor releases! Can I use this?*

> Sure, go nuts! But don't say I didn't warn you when literally all of your
> code stops compiling...

*I found something that doesn't work, a missing OpenGL function, a typo in
the docs where you spelled it 'indicies', or an API that is dumb and breaks
the borrow checker. Where can I yell at you about it?*

> Feel free [file an issue](https://github.com/kylewlacy/glitter/issues)! I'm
> open to suggestions for API changes or ideas as well!

# License
All source code in this repository is licensed under both the MIT license
and the Apache 2.0 license, at the choice of the library user. All code in
the [`examples`](https://github.com/kylewlacy/tree/master/examples)
directory is *additionally* available in the public domain, under the
terms of the Unlicense. See the appropriate `LICENSE` files for some legal
words.
