use ge::events::handle_events;
use ge::init::{init, Game};
use ge::player::Player;
use ge::resource_manager::ResourceManager;
use ge::traits::Renderable;

use glium::Surface;

mod block;
use block::Block;

fn main() {
    // init game and destructure
    let game = init();

    // destructure fields off Game
    let Game {
        display,
        event_loop,
        mut resource_manager,
        ..
    } = game;

    // initialize blocks
    Block::init(&display, &mut resource_manager);

    // load block texture
    let texture_name = String::from("block");
    let texture = ResourceManager::load_texture(&display, "textures/awesomeface.png");
    resource_manager.add_texture(&texture_name, texture);

    let mut blocks: Vec<Block> = vec![];

    for x in 0..8 {
        for y in 0..4 {
            // define block
            let mut block = Block::new(
                cgmath::Vector2::new(x as f32 / 8.0, y as f32 / 12.0),
                cgmath::Vector2::new(1.0 / 8.0, 1.0 / 12.0),
                image::Rgba::from([1.0, 0.0, 0.0, 1.0]),
            );
            // set block texture
            block.set_texture(texture_name.clone());
            blocks.push(block);
        }
    }

    // define player
    Player::init(&display, &mut resource_manager);
    let mut player = Player::new(
        cgmath::Vector2::new(400.0 / 800.0, 500.0 / 600.0),
        cgmath::Vector2::new(100.0 / 800.0, 40.0 / 800.0),
        image::Rgba::from([1.0, 1.0, 1.0, 1.0]),
    );

    // load player texture
    let texture_name = String::from("player");
    let texture = ResourceManager::load_texture(&display, "textures/paddle.png");
    resource_manager.add_texture(&texture_name, texture);

    player.set_texture(String::from("player"));

    // game loop
    event_loop.run(move |ev, _, control_flow| {
        // handle events, keyboard input, etc.
        handle_events(ev, control_flow);

        let mut frame = display.draw();
        frame.clear_color(0.2, 0.3, 0.3, 1.0);

        // DRAW START
        for block in blocks.iter() {
            block.draw(&mut frame, &resource_manager).unwrap();
        }

        player.draw(&mut frame, &resource_manager).unwrap();

        frame.finish().unwrap();
        // DRAW END
    });
}
