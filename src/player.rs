use pancurses::{Input};
use weg_ecs::registry::{ Link, Registry };
use render::Render;
use component::Component;
use utils::Vec2;

pub struct Player<'a> {
    render: &'a Render,
    link: Link,
}

impl<'a> Player<'a> {
    pub fn new(x: i32, y: i32, render: &'a Render, registry: &mut Registry<Component>) -> Self {
    use Component::*;
        let link = registry.link_make_entity(vec![
                                             Char('@'),
                                             Pos(Vec2{x,y}),
                                             Vel(Vec2{x:0,y:0})]);
        Player {
            render,
            link,
        }
    }

    pub fn input(&self, registry: &mut Registry<Component>) -> bool {
        let mut quit = false;
        let mut vel: Option<Vec2<i32>> = None;
        for component in registry.get_entity_by_link(self.link) {
            match *component {
                Component::Vel(v) => { vel = Some(v) },
                _ => (),
            }
        }
        let mut vel = vel.unwrap();

        let inp = self.render.getch();
        match inp {
            //TODO send quit signal to loop once loop is complete.
            Some(Input::Character('h')) => {vel.set(0 ,-1)},
            Some(Input::Character('j')) => {vel.set(0 , 1)},
            Some(Input::Character('l')) => {vel.set(1 , 0)},
            Some(Input::Character('k')) => {vel.set(-1, 0)},
            Some(Input::Character(c)) if (c == 'q' || c == 'Q') => {quit = true},
            Some(_) => (),
            None => {vel.set(0,0)},
        }
        quit
    }
}
