// Define a 3D integer array with dimensions 8x8x8
const ARRAY_SIZE: usize = 8;
type CubeArray = [[[i32; ARRAY_SIZE]; ARRAY_SIZE]; ARRAY_SIZE];

// Modify create_box function to accept and render cubes based on the array
fn create_box(display: &glium::Display<WindowSurface>, cube_array: &CubeArray) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Iterate over the cube array and render cubes based on the values
    for x in 0..ARRAY_SIZE {
        for y in 0..ARRAY_SIZE {
            for z in 0..ARRAY_SIZE {
                if cube_array[x][y][z] == 1 {
                    let offset_x = x as f32 - (ARRAY_SIZE as f32 / 2.0);
                    let offset_y = y as f32 - (ARRAY_SIZE as f32 / 2.0);
                    let offset_z = z as f32 - (ARRAY_SIZE as f32 / 2.0);

                    // Define vertices for cube at current position
                    let cube_vertices = &[
                        Vertex { position: [offset_x + 0.5, offset_y - 0.5, offset_z - 0.5, 1.0], normal: [0.0, 0.0, 0.0, 0.0] },
                        Vertex { position: [offset_x - 0.5, offset_y - 0.5, offset_z - 0.5, 1.0], normal: [0.0, 0.0, 0.0, 0.0] },
                        Vertex { position: [offset_x - 0.5, offset_y + 0.5, offset_z - 0.5, 1.0], normal: [0.0, 0.0, 0.0, 0.0] },
                        Vertex { position: [offset_x + 0.5, offset_y + 0.5, offset_z - 0.5, 1.0], normal: [0.0, 0.0, 0.0, 0.0] },
                        // Define other vertices for the cube similarly
                        // ...
                    ];

                    let start_index = vertices.len() as u16;
                    for v in cube_vertices {
                        vertices.push(*v);
                    }

                    // Define indices for the cube
                    let cube_indices = &[
                        start_index, start_index + 1, start_index + 2,
                        start_index, start_index + 2, start_index + 3,
                        // Define other indices for the cube similarly
                        // ...
                    ];

                    for idx in cube_indices {
                        indices.push(*idx);
                    }
                }
            }
        }
    }

    let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();
    (vertex_buffer, index_buffer)
}







struct Application {
    // Other fields remain unchanged
    cube_array: CubeArray,
}

impl ApplicationContext for Application {
    fn new(display: &Display<WindowSurface>) -> Self {
        // Initialize the cube array
        let cube_array = [[[1; ARRAY_SIZE]; ARRAY_SIZE]; ARRAY_SIZE]; // Example: All cubes rendered

        // Other initialization code remains unchanged
        // ...

        Self {
            // Other fields remain unchanged
            cube_array,
        }
    }

    fn draw_frame(&mut self, display: &Display<WindowSurface>) {
        // Render cubes based on the cube array
        let (model_vertex_buffer, model_index_buffer) = create_box(display, &self.cube_array);

        // Drawing logic remains unchanged
        // ...
    }

    // Other methods remain unchanged
    // ...
}









use rand::prelude::*;

// Inside the `new` method of the `Application` struct
fn new(display: &Display<WindowSurface>) -> Self {
    // Initialize the cube array with random values
    let mut rng = rand::thread_rng();
    let mut cube_array = [[[0; ARRAY_SIZE]; ARRAY_SIZE]; ARRAY_SIZE];
    for x in 0..ARRAY_SIZE {
        for y in 0..ARRAY_SIZE {
            for z in 0..ARRAY_SIZE {
                cube_array[x][y][z] = rng.gen_range(0..=1); // Random values 0 or 1
            }
        }
    }

    // Other initialization code remains unchanged
    // ...

    Self {
        // Other fields remain unchanged
        cube_array,
    }
}