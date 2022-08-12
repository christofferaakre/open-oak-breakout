use cgmath::Vector2;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Block {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub id: uuid::Uuid,
    pub texture_name: String,
    pub color: image::Rgba<f32>,
}

use glium::vertex::VertexBuffer;

use ge::structs::Vertex;
use ge::traits::Renderable;

impl Block {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, color: image::Rgba<f32>) -> Self {
        let block = Block {
            position,
            size,
            id: Uuid::new_v4(),
            texture_name: Default::default(),
            color,
        };

        return block;
    }
}

impl Renderable for Block {
    fn get_name() -> String {
        String::from("block")
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn texture_name(&self) -> String {
        self.texture_name.clone()
    }

    fn size(&self) -> Vector2<f32> {
        self.size
    }

    fn position(&self) -> cgmath::Vector2<f32> {
        self.position
    }

    fn color(&self) -> image::Rgba<f32> {
        self.color
    }

    fn get_vertex_buffer(display: &glium::Display) -> VertexBuffer<Vertex> {
        VertexBuffer::new(display, &VERTICES).unwrap()
    }

    fn get_program(display: &glium::Display) -> glium::Program {
        let vertex_src = std::fs::read_to_string("shaders/block.vs").unwrap();

        let fragment_src = std::fs::read_to_string("shaders/block.fs").unwrap();

        let program =
            glium::Program::from_source(display, vertex_src.as_str(), fragment_src.as_str(), None)
                .unwrap();

        return program;
    }

    fn set_texture(&mut self, texture_name: String) {
        self.texture_name = texture_name;
    }
}

pub const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-0.5, -0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5],
        tex_coords: [1.0, 1.0],
    },
];
