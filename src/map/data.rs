extern crate tcod;

use ::std::cmp;
use self::tcod::colors::Color;
use self::tcod::map::{FovAlgorithm};

// size of the map
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

//parameters for dungeon generator
pub const ROOM_MAX_SIZE: i32 = 10;
pub const ROOM_MIN_SIZE: i32 = 6;
pub const MAX_ROOMS: i32 = 30;
const MAX_ROOM_MONSTERS: i32 = 3;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;  // default FOV algorithm
const FOV_LIGHT_WALLS: bool = true;  // light walls or not
const TORCH_RADIUS: i32 = 10;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color { r: 130, g: 110, b: 50 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };
const COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50 };

pub type Map = Vec<Vec<Tile>>;

pub struct Dungeon {
    pub map: Map,
    pub width: i32,
    pub height: i32,
    pub player_start: (i32, i32),
    pub rooms: Vec<Rect>
}
impl Dungeon {
    pub fn new() -> Dungeon {
        Dungeon {
            height: MAP_HEIGHT,
            map: vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize],
            width: MAP_WIDTH,
            player_start: (0, 0),
            rooms: vec![]
        }
    }
    pub fn create_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        // horizontal tunnel. `min()` and `max()` are used in case `x1 > x2`
        for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
            self.map[x as usize][y as usize] = Tile::empty();
        }
    }
    pub fn create_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        // vertical tunnel
        for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
            self.map[x as usize][y as usize] = Tile::empty();
        }
    }
    pub fn create_room(&mut self, room: Rect) {
        // go through the tiles in the rectangle and make them passable
        for x in (room.x1 + 1)..room.x2 {
            for y in (room.y1 + 1)..room.y2 {
                self.map[x as usize][y as usize] = Tile::empty();
            }
        }
}
}

/// A tile of the map and its properties
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub background: Color,
    pub blocked: bool,
    explored: bool,
    block_sight: bool,
}
impl Tile {
    pub fn empty() -> Self {
        Tile{background: COLOR_DARK_GROUND, blocked: false, explored: false, block_sight: false}
    }

    pub fn wall() -> Self {
        Tile{background: COLOR_DARK_WALL, blocked: true, explored: false, block_sight: true}
    }
}

/// A rectangle on the map, used to characterise a room.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}
impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect { x1: x, y1: y, x2: x + w, y2: y + h }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        // returns true if this rectangle intersects with another one
        (self.x1 <= other.x2) && (self.x2 >= other.x1) &&
            (self.y1 <= other.y2) && (self.y2 >= other.y1)
    }
}
