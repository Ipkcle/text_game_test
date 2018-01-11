use utils::Vec2;
use enumset::EnumSet;
use utils;

pub enum Component {
    Pos(Vec2<i32>),
    Char(char),
    Vel(Vec2<i32>),
}

enum_set_type! {
    pub enum Flag {
        Pos,
        Char,
        Vel,
    }
}

impl Component {
    fn get_flag(&self) -> Flag {
        use self::Component::*;
        match *self {
            Pos(..) => Flag::Pos,
            Char(..) => Flag::Char,
            Vel(..) => Flag::Vel,
            _ => panic!(),
        }
    }
}

fn entity_set(entity: &[Component]) -> EnumSet<Flag> {
    let mut flag_set: EnumSet<Flag> = EnumSet::new();
    for component in entity {
        flag_set.insert(component.get_flag());
    }
    flag_set
}

pub fn entity_matches(set: EnumSet<Flag> , entity: &[Component]) -> bool {
    set.is_subset(entity_set(entity))
}
