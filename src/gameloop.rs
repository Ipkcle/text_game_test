use weg_ecs::registry::{ Link, Registry };
use pancurses::endwin;
use map::{Map, Tile};
use render::Render;
use component::Component;
use player::Player;
use physics::physics;

pub struct GameLoop {
    should_quit: bool,
}

impl GameLoop {
    pub fn new() -> Self {
        GameLoop {
            should_quit: false,
        }
    }

    pub fn run(&mut self) {
        let mut map = Map::new(1000, 1000);
        map.set_range(0,1000,0,1000,Tile::Wall);
        map.set_range(1,200,1,20,Tile::Room);
        map.set_range(180,200,20,400,Tile::Room);
        let mut registry: Registry<Component> = Registry::new();
        let render = Render::new();
        let player = Player::new(10,10,&render, &mut registry);
        loop {
            if self.should_quit { break }
            //get_input
            self.should_quit = player.input(&mut registry);
            //run physics
            physics(&mut registry, &render);
            //render
            render.render_map(&map);
            render.render_entities(&mut registry);
            render.refresh();
        }
        endwin();
    }
}
