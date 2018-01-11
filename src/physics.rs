use weg_ecs::registry::{ Link, Registry };
use enumset::EnumSet;
use component::{ Component, Flag };
use component;
use utils::Vec2;
use render::Render;

const SET: EnumSet<Flag> = enum_set!(Flag, Flag::Pos | Flag::Vel);

pub fn physics(registry: &mut Registry<Component>, render: &Render) {
    // 10 lines of boilerplate for every new system, even simple ones. must be some solution.
    use self::Component::*;//
    let mut stream = registry.stream();//
    For!(entity in stream => {//
        if component::entity_matches(SET, entity) {//
            let mut pos: Option<&mut Vec2<i32>> = None;
            let mut vel: Option<Vec2<i32>> = None;
            for component in entity {//
                match *component {//
                    Pos(ref mut p) => { pos = Some(p) },
                    Vel(v) => { vel = Some(v) },
                    _ => (),
                }//
            }//
            let mut pos = pos.unwrap();
            let mut vel = vel.unwrap();
            pos.x = pos.x + vel.x;
            pos.y = pos.y + vel.y;
            render.debug_print("abc");
        }//
    })//
}
