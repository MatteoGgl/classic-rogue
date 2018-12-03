extern crate tcod;

use self::tcod::colors::Color;

/// This is a generic object: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen.
#[derive(Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub char: char,
    pub color: Color,
    name: String,
    blocks: bool,
    alive: bool,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color, name: String) -> Object {
        Object {
            x, y,
            char,
            color,
            name,
            blocks: false,
            alive: true
        }
    }
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

pub fn create_player(objects: &mut Vec<Object>, position: (i32, i32) ) {
    let (x, y) = position;
    let player = Object::new(x, y, '@', tcod::colors::WHITE, "Player".to_owned());
    objects.push(player);
}
