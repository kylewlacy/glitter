# glitter - A safe, low-level, zero-cost Rust OpenGL wrapper library

glitter was an experimental library I made to attempt to codify the rules of
OpenGL's state machine with Rust's type system. The goal was to have a zero-cost
and safe wrapper around the OpenGL API, where invalid OpenGL calls would be
caught at compile-time instead of runtime. The core idea is to represent OpenGL
state using [typestate](https://yoric.github.io/post/rust-typestate/).

I only got as far as wrapping a subset of OpenGL ES 2. While working on the
project, I was held back by a number of Rust limitations, some of which have
since been lifted (such as const generics), and others that still linger to this
day (like [partial borrowing](https://github.com/rust-lang/rfcs/issues/1215)).

Since I initially published glitter, the landscape of graphics APIs has evolved
greatly, both in the Rust community and in the world at large. I no longer feel
that glitter is a good fit for any kind of project-- gamedevs are better served
by higher-level abstractions (like the renderer integrated in [Bevy](https://bevyengine.org/)),
engine devs are better served with platform-agnostic abstractions (like [wgpu](https://wgpu.rs/)),
and library devs are served better by direct access to graphics APIs
without needing to jump through type-system hoops. Modern graphics APIs like
Vulkan, Metal, and DX12 also seem to steer away from the stateful design of
OpenGL, which makes something like glitter less compelling or interesting.

So, I've decided to put glitter into archive mode. I hope it will be of interest
to those looking for inspiration in the future!

**Note that this repo won't get any security patches! There are currently known
security holes in some of the dependencies, so it should not be used for any
serious projects!**

# Code examples

There are some examples of glitter in the [`examples`](https://github.com/kylewlacy/glitter/tree/master/examples)
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
such as with the [`VertexBuffer`](https://docs.rs/glitter/0.1.2/glitter/vertex_buffer/struct.VertexBuffer.html)
type, which provides a low-overhead, high-level interface for creating,
buffering, and drawing vertex data.

## Zero Cost
Many of the core types in glitter are composed of [zero-sized types](https://doc.rust-lang.org/nomicon/vec-zsts.html)
or [pointers to zero-sized types](http://www.wabbo.org/blog/2014/03aug_09aug.html)
(such as the [`Context`](https://docs.rs/glitter/0.1.2/glitter/context/type.Context.html)
type). This means that glitter method calls can often compile directly OpenGL
function invocations.

----------

For more details about how glitter achieves its design goals, consult
the [documentation](https://docs.rs/glitter/0.1.2/glitter/index.html).

# License
All source code in this repository is licensed under both the MIT license
and the Apache 2.0 license, at the choice of the library user. All code in
the [`examples`](https://github.com/kylewlacy/glitter/tree/master/examples)
directory is *additionally* available in the public domain, under the
terms of the Unlicense. See the appropriate `LICENSE` files for some legal
words.
