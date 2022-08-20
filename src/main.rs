use glium::glutin::event::{self, ElementState, KeyboardInput, VirtualKeyCode};
use glium::Surface;

use cgmath::Vector2;
use open_oak::events::handle_events;
use open_oak::init::{init, Game};
use open_oak::rectangle::Rectangle;
use open_oak::resource_manager::ResourceManager;
use open_oak::traits::{Renderable, Shaders, Texture};

use levels::BlockType;

use std::collections::HashSet;

use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct Block {
    rect: Rectangle,
}

#[derive(Debug, Clone)]
struct Player {
    rect: Rectangle,
    velocity: Vector2<f32>,
    position: Vector2<f32>,
}

impl Player {
    fn new(position: Vector2<f32>, size: Vector2<f32>) -> Self {
        Player {
            position,
            velocity: Vector2::new(0.0, 0.0),
            rect: Rectangle::new(position, size, image::Rgba([1.0, 1.0, 1.0, 1.0])),
        }
    }

    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.rect.position = self.position;
    }
}

mod levels;

impl Block {
    fn new(position: Vector2<f32>, size: Vector2<f32>, color: image::Rgba<f32>) -> Block {
        let rect = Rectangle::new(position, size, color);
        Block { rect }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init game and destructure
    let game = init();

    // destructure fields off Game
    let Game {
        display,
        event_loop,
        mut resource_manager,
        ..
    } = game;

    // init rectangle
    Rectangle::init(&mut resource_manager, &display);

    // load breakable texture
    let texture_name = String::from("breakable");
    let texture = ResourceManager::load_texture(&display, "textures/block.png");
    resource_manager.add_texture(&texture_name, texture);

    // load solid texture
    let texture_name = String::from("solid");
    let texture = ResourceManager::load_texture(&display, "textures/block_solid.png");
    resource_manager.add_texture(&texture_name, texture);

    let level = levels::parse_file_to_level("levels/level1.lvl")?;
    let mut blocks = vec![];
    for (y, row) in &mut level.blocks.iter().enumerate() {
        for (x, block_type) in row.iter().enumerate() {
            let mut block = Block::new(
                Vector2::new(
                    x as f32 / level.width as f32,
                    y as f32 / level.height as f32 / 3.0,
                ),
                Vector2::new(1.0 / level.width as f32, 1.0 / level.height as f32 / 3.0),
                image::Rgba([1.0, 1.0, 1.0, 1.0]),
            );
            match block_type {
                BlockType::Breakable => {
                    block.rect.set_texture(String::from("breakable"));
                    block.rect.color = image::Rgba([255.0 / 255.0, 152.0 / 255.0, 0.0, 1.0])
                }
                BlockType::Solid => {
                    block.rect.set_texture(String::from("solid"));
                    block.rect.color =
                        image::Rgba([158.0 / 255.0, 158.0 / 255.0, 158.0 / 255.0, 1.0]);
                }
            }
            blocks.push(block);
        }
    }

    // load player texture
    let player_texture_name = String::from("player");
    let texture = ResourceManager::load_texture(&display, "textures/paddle.png");
    resource_manager.add_texture(&player_texture_name, texture);

    // init player
    let mut player = Player::new(Vector2::new(0.35, 1.0 - 0.1), Vector2::new(0.3, 0.06));
    player.rect.set_texture(player_texture_name);

    let mut pressed_keys: HashSet<VirtualKeyCode> = HashSet::new();

    let mut last_frame = Instant::now();
    // game loop
    event_loop.run(move |ev, _, control_flow| {
        // calculate time since last frame
        let dt = last_frame.elapsed();
        last_frame += dt;
        println!("dt: {:?}", dt);

        // handle events, keyboard input, etc.
        let keyboard_input = handle_events(ev, control_flow);
        if let Some(keyboard_input) = keyboard_input {
            handle_keyboard_input(keyboard_input, &mut pressed_keys, &mut player);
        }

        println!("Pressed keys: {:?}", pressed_keys);

        player.update(dt.as_secs_f32());

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.3, 0.3, 1.0);

        // DRAW START
        for block in &blocks {
            block.rect.draw(&mut frame, &resource_manager).unwrap();
        }

        player.rect.draw(&mut frame, &resource_manager).unwrap();

        frame.finish().unwrap();
        // DRAW END
    });
}

fn handle_keyboard_input(
    input: event::KeyboardInput,
    pressed_keys: &mut HashSet<VirtualKeyCode>,
    player: &mut Player,
) {
    let keycode = input
        .virtual_keycode
        .unwrap_or_else(|| panic!("Keyboard input {:?} did not have a valid keycode", input));

    // key pressed and wasn't pressed before
    if input.state == ElementState::Pressed && !pressed_keys.contains(&keycode) {
        pressed_keys.insert(keycode);
    }
    // key was pressed before and was released
    if input.state == ElementState::Released && pressed_keys.contains(&keycode) {
        pressed_keys.remove(&keycode);
    }
    match keycode {
        event::VirtualKeyCode::Escape => {
            std::process::exit(0);
        }
        _ => {
            println!("{:?}", input);
        }
    };

    player.velocity = Vector2::new(0.0, 0.0);

    if pressed_keys.contains(&VirtualKeyCode::A) {
        player.velocity += Vector2::new(-1.0, 0.0);
    }

    if pressed_keys.contains(&VirtualKeyCode::D) {
        player.velocity += Vector2::new(1.0, 0.0);
    }
}
