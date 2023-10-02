// Uncomment these following global attributes to silence most warnings of "low" interest:
/*
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]
*/
extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

mod shader;
mod util;
mod mesh;
mod scene_graph;

use gl::UniformMatrix3fv;
use glm::vec3;
use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;
use scene_graph::SceneNode;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //

// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  pointer_to_array(my_array)
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()


// == // Generate your VAO here
unsafe fn create_vao(vertices: &Vec<f32>, colors: &Vec<f32>, indices: &Vec<u32>, normals: &Vec<f32>) -> u32 {
    /***FOR VERTICES***/
    // * Generate a VAO for verticies and bind it
    let mut vao: u32 = 0;
    unsafe {gl::GenVertexArrays(1, &mut vao);} //generating a vao and storing a reference to it in a variable
    unsafe {gl::BindVertexArray(vao);} //binding this vao to signal this is the one currently worked on

    // * Generate a VBO for vertices and bind it
    let mut vbo_v: u32 = 0;
    unsafe {gl::GenBuffers(1, &mut vbo_v);}
    unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, vbo_v);}

    // * Fill vbo_v with data
    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            byte_size_of_array(vertices),
            pointer_to_array(vertices),
            gl::STATIC_DRAW
        );
    }

    // * Configure a VAP for the vertex data and enable it
    let vap_index_v: u32 = 0;
    unsafe {
        gl::VertexAttribPointer(
            vap_index_v, 
            3, //amount of verticies, 2 if 2d and 3 if 3d
            gl::FLOAT, //type of data
            gl::FALSE,
            offset::<f32>(3) as i32, //buffer contains x, y, z
            0 as *const c_void
        );
    }

    /***FOR COLORS***/
    // * Generate a VBO for colors and bind it
    let mut vbo_c: u32 = 0;
    unsafe {gl::GenBuffers(1, &mut vbo_c);}
    unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, vbo_c);}

    // * Fill vbo_c with data
    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            byte_size_of_array(colors),
            pointer_to_array(colors),
            gl::STATIC_DRAW
        );
    }

    // * Configure a VAP for the color data and enable it
    let vap_index_c: u32 = 1;
    unsafe{
        gl::VertexAttribPointer(
            vap_index_c, 
            4, //r, g, b, a
            gl::FLOAT, //type of data
            gl::FALSE,
            offset::<f32>(4) as i32, //buffer contains r, g, b, a and each is the size of float
            0 as *const c_void
        );
    }

    /***FOR NORMALS***/
    // * Generate a VBO for normals and bind it
    let mut vbo_n: u32 = 0;
    unsafe {gl::GenBuffers(1, &mut vbo_n);}
    unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, vbo_n);}

    // * Fill vbo_n with data
    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            byte_size_of_array(normals),
            pointer_to_array(normals),
            gl::STATIC_DRAW
        );
    }

    // * Configure a VAP for the color data and enable it
    let vap_index_n: u32 = 2;
    unsafe{
        gl::VertexAttribPointer(
            vap_index_n, 
            3, //x, y, z
            gl::FLOAT, //type of data
            gl::FALSE,
            offset::<f32>(3) as i32, //buffer contains x, y, z and each is the size of float
            0 as *const c_void
        );
    }

    //Enable vertex attrib array for both vap
    unsafe {
        gl::EnableVertexAttribArray(vap_index_v);
        gl::EnableVertexAttribArray(vap_index_c);
        gl::EnableVertexAttribArray(vap_index_n);
    }

    // * Generate a IBO and bind it
    let mut ibo: u32 = 0;
    unsafe {gl::GenBuffers(1, &mut ibo);}
    unsafe {gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);}

    // * Fill it with data
    unsafe {
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER, 
            byte_size_of_array(indices), 
            pointer_to_array(indices), 
            gl::STATIC_DRAW
        );
    }

    // * Return the ID of the VAO

    vao
}


fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(INITIAL_SCREEN_W, INITIAL_SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        // == // Set up your VAO around here

        let lunar_mesh = mesh::Terrain::load("./resources/lunarsurface.obj");
        let lunar_mesh_vao = unsafe {
            create_vao(&lunar_mesh.vertices, &lunar_mesh.colors, &lunar_mesh.indices, &lunar_mesh.normals)
        };

        let helicopter_mesh = mesh::Helicopter::load("./resources/helicopter.obj");
        let helicopter_mesh_body_vao = unsafe{create_vao(
            &helicopter_mesh.body.vertices, 
            &helicopter_mesh.body.colors, 
            &helicopter_mesh.body.indices, 
            &helicopter_mesh.body.normals)
        };
        let helicopter_mesh_door_vao = unsafe{create_vao(
            &helicopter_mesh.door.vertices, 
            &helicopter_mesh.door.colors, 
            &helicopter_mesh.door.indices, 
            &helicopter_mesh.door.normals)
        };
        let helicopter_mesh_main_rotor_vao = unsafe{create_vao(
            &helicopter_mesh.main_rotor.vertices, 
            &helicopter_mesh.main_rotor.colors, 
            &helicopter_mesh.main_rotor.indices, 
            &helicopter_mesh.main_rotor.normals)
        };
        let helicopter_mesh_tail_rotor_vao = unsafe{create_vao(
            &helicopter_mesh.tail_rotor.vertices, 
            &helicopter_mesh.tail_rotor.colors, 
            &helicopter_mesh.tail_rotor.indices, 
            &helicopter_mesh.tail_rotor.normals)
        };

        /**** Generating scene nodes here ****/
        let mut root_node = SceneNode::new(); 

        let mut lunar_node = SceneNode::from_vao(lunar_mesh_vao, lunar_mesh.index_count);
        
        let mut helicopter_body_node = SceneNode::from_vao(helicopter_mesh_body_vao, helicopter_mesh.body.index_count);
        let mut helicopter_door_node = SceneNode::from_vao(helicopter_mesh_door_vao, helicopter_mesh.door.index_count);
        let mut helicopter_main_rotor_node = SceneNode::from_vao(helicopter_mesh_main_rotor_vao, helicopter_mesh.main_rotor.index_count);
        let mut helicopter_tail_rotor_node = SceneNode::from_vao(helicopter_mesh_tail_rotor_vao, helicopter_mesh.tail_rotor.index_count);
        
        root_node.add_child(&lunar_node); //Adding lunar node to root
        lunar_node.add_child(&helicopter_body_node); //Adding helicopter to lunar node
        helicopter_body_node.add_child(&helicopter_door_node); //Adding all parts of helicopter to helicopter...
        helicopter_body_node.add_child(&helicopter_main_rotor_node);
        helicopter_body_node.add_child(&helicopter_tail_rotor_node);

        // == // Set up your shaders here

        let simple_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
        };
        unsafe {
            simple_shader.activate();
        }
        
        /**** CAMERA MATRIX INITIALIZATIONS ****/
        //Creating a perspective matrix with same aspect ratio as window, fov at 90 deg and clipping plane in [-1, -1000]
        let perspective_m: glm::Mat4 = glm::perspective(
            window_aspect_ratio,
            glm::pi::<f32>() / 2.0,
            1.0,
            1000.0
        );

        //Camera position along each axis, initially at -1 on z-axis to see all triangles
        let mut cam_translation: glm::Vec3 = glm::Vec3::new(0.0, 0.0, -1.0);
        let mut cam_rotation: glm::Vec2 = glm::Vec2::new(0.0, 0.0);
        
        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut prevous_frame_time = first_frame_time;
        loop {
            //Initialising the camera transformation as the identity matrix
            let mut cam_transformation: glm::Mat4 = glm::identity();

            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(prevous_frame_time).as_secs_f32();
            prevous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
                    println!("Resized");
                    unsafe { gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32); }
                }
            }

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html

                        //Movement in x-direction
                        VirtualKeyCode::A => {
                            cam_translation.x += 100.0 * delta_time;
                        }
                        VirtualKeyCode::D => {
                            cam_translation.x -= 100.0 * delta_time;
                        }
                        //Movement in y-direction
                        VirtualKeyCode::LShift => {
                            cam_translation.y += 100.0 * delta_time;
                        }
                        VirtualKeyCode::Space => {
                            cam_translation.y -= 100.0 * delta_time;
                        }
                        //Movement in z-direction
                        VirtualKeyCode::W => {
                            cam_translation.z += 100.0 * delta_time;
                        }
                        VirtualKeyCode::S => {
                            cam_translation.z -= 100.0 * delta_time;
                        }
                        //Rotate camera along y
                        VirtualKeyCode::Right => {
                            cam_rotation.y += delta_time;
                        }
                        VirtualKeyCode::Left => {
                            cam_rotation.y -= delta_time;
                        }
                        //Rotate camera along x
                        VirtualKeyCode::Up => {
                            cam_rotation.x -= delta_time;
                        }
                        VirtualKeyCode::Down => {
                            cam_rotation.x += delta_time;
                        }

                        // default handler:
                        _ => { }
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {

                // == // Optionally access the acumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }

            //Calculating the transformation on the geometry
            cam_transformation = glm::translation(&cam_translation) * cam_transformation; //Translate
            cam_transformation = glm::rotation( cam_rotation.x, &glm::vec3(1.0, 0.0, 0.0)) * cam_transformation; //Rotate about x
            cam_transformation = glm::rotation( cam_rotation.y, &glm::vec3(0.0, 1.0, 0.0)) * cam_transformation; //Rotate about y

            unsafe {
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                //Updating uniform containing 4x4 matrix
                gl::UniformMatrix4fv(0, 1, gl::FALSE, (perspective_m*cam_transformation).as_ptr());

                // == // Issue the necessary gl:: commands to draw your scene here
                //Lunar mesh
                gl::BindVertexArray(lunar_mesh_vao);
                gl::DrawElements(gl::TRIANGLES, lunar_mesh.indices.len() as i32, gl::UNSIGNED_INT, 0 as *const c_void);

                //Different parts of helicopter mesh
                gl::BindVertexArray(helicopter_mesh_body_vao);
                gl::DrawElements(gl::TRIANGLES, helicopter_mesh.body.indices.len() as i32, gl::UNSIGNED_INT, 0 as *const c_void);
                gl::BindVertexArray(helicopter_mesh_door_vao);
                gl::DrawElements(gl::TRIANGLES, helicopter_mesh.door.indices.len() as i32, gl::UNSIGNED_INT, 0 as *const c_void);
                gl::BindVertexArray(helicopter_mesh_main_rotor_vao);
                gl::DrawElements(gl::TRIANGLES, helicopter_mesh.main_rotor.indices.len() as i32, gl::UNSIGNED_INT, 0 as *const c_void);
                gl::BindVertexArray(helicopter_mesh_tail_rotor_vao);
                gl::DrawElements(gl::TRIANGLES, helicopter_mesh.tail_rotor.indices.len() as i32, gl::UNSIGNED_INT, 0 as *const c_void);
            }

            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });


    // == //
    // == // From here on down there are only internals.
    // == //


    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::Resized(physical_size), .. } => {
                println!("New window size! width: {}, height: {}", physical_size.width, physical_size.height);
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => { *control_flow = ControlFlow::Exit; }
                    Q      => { *control_flow = ControlFlow::Exit; }
                    _      => { }
                }
            }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => { }
        }
    });
}
