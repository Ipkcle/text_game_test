use pancurses::{Window, initscr, endwin, Input, noecho};
use weg_ecs::registry::{ Link, Registry };
use enumset::EnumSet;
use component::{ Component, Flag };
use component;
use map::Map;
use utils::Vec2;

const SET: EnumSet<Flag> = enum_set!(Flag, Flag::Pos | Flag::Char);

struct Camera {
    x: u32,
    y: u32,
}

impl Camera { pub fn new() -> Camera { Camera{x:0,y:0} } }

pub struct Render {
    window: Window,
    camera: Camera,
}

impl Render {
    pub fn refresh(&self) {
        self.window.refresh();
    }

    pub fn render_entities(&self, registry: &mut Registry<Component>) {
        use self::Component::*;
        let mut stream = registry.stream();
        For!(entity in stream => {
            if component::entity_matches(SET, entity) {
                let mut pos: Option<Vec2<i32>> = None;
                let mut ch: Option<char> = None;
                for component in entity {
                    match *component {
                        Pos(p) => {pos = Some(p)},
                        Char(c) => {ch = Some(c)},
                        _ => (),
                    }
                }
                let col = pos.unwrap().x - self.camera.x as i32;
                let row = pos.unwrap().y - self.camera.y as i32;
                self.window.mvaddch(col, row, ch.unwrap());
            }
        })
    }
    
    pub fn getch(&self) -> Option<Input> {
        self.window.getch()
    }

    pub fn debug_print(&self, string: &str) {
        self.window.mvprintw(0,0,string);
    }

    pub fn render_map(&self, map: &Map) {
        let (width, height) = self.window.get_max_yx();
        for col in 0..width as i32 {
            for row in 0..height as i32 {
                let ch = map.get(row as u32 + self.camera.x, col as u32 + self.camera.y).get_char();
                self.window.mvaddch(col, row, ch);
            }
        }
    }
    
    pub fn set_camera(&mut self, x: u32, y: u32) {
        self.camera.x = x;
        self.camera.y = y;
    }

    fn initwin() -> Window {
        let window = initscr();
        window.keypad(true);
        noecho();
        window
    }

    pub fn new() -> Render {
        Render{ window: Render::initwin(), camera: Camera::new() }
    }
}

    
