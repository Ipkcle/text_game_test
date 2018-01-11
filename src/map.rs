pub struct Map {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        Map {
            width,
            height,
            tiles: vec![Tile::Void; (width*height) as usize],
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (x + (y * self.width)) as usize
    }

    pub fn set(&mut self, x: u32, y: u32, tile: Tile) {
        let index = self.get_index(x, y);
        self.tiles[index] = tile;
    }

    pub fn get(&self, x: u32, y: u32) -> Tile {
        self.tiles[self.get_index(x, y)]
    }

    pub fn set_range(&mut self, x0: u32, x1: u32, y0: u32, y1: u32, tile: Tile) {
        for x in x0..x1 {
            for y in y0..y1 {
                self.set(x, y, tile);
            }
        }
    }

}

#[derive(Clone)]
#[derive(Copy)]
pub enum Tile {
    Room,
    Hallway,
    Wall,
    Void,
}

impl Tile {
    pub fn get_char(&self) -> char {
        use self::Tile::*;
        match *self {
            Room => '.',
            Hallway => '.',
            Wall => '#',
            Void => ' ',
        }
    }
}
