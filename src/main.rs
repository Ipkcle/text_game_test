#[macro_use]
extern crate weg_ecs;
#[macro_use]
extern crate enumset;
extern crate pancurses;

use pancurses::{Window, initscr, endwin, Input, noecho};
use weg_ecs::registry::{ Link, Registry };

mod render;
mod component;
mod map;
mod utils;
mod physics;
mod player;
mod gameloop;
use utils::Vec2;
use map::{Map, Tile};
use render::Render;
use component::Component;

fn move_at() {
    let window: Window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();

    let ch = '@';
    let mut x = 0;
    let mut y = 0;

    loop {
        window.mvaddch(y, x, ch);
        pancurses::curs_set(0);
        let inp = window.getch();
        window.mvaddch(y, x, ' ');
        match inp {
            Some(Input::Character(c)) if (c == 'q' || c == 'Q') => break,
            Some(Input::Character('h')) => {x -= 1},
            Some(Input::Character('j')) => {y += 1},
            Some(Input::Character('l')) => {x += 1},
            Some(Input::Character('k')) => {y -= 1},
            Some(_) => (),
            None => ()
        }
    }

    endwin();
}

fn other_test() {
    let mut map = Map::new(1000, 1000);
    let mut registry: Registry<Component> = Registry::new();
    registry.make_entity(vec![Component::Char('@'), Component::Pos(Vec2{x: 10, y: 10})]);
    map.set_range(0,1000,0,1000,Tile::Wall);
    map.set_range(2,22,2,22,Tile::Room);
    let render = Render::new();
    render.render_map(&map);
    render.render_entities(&mut registry);
    render.refresh();
    loop {
        let inp = render.getch();
        if inp == Some(Input::Character('q')) {
            break
        }
    }
    endwin();
}
fn main() {
    let mut l = gameloop::GameLoop::new();
    l.run();
}
