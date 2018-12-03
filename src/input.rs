extern crate tcod;

use tcod::input::Key;
use tcod::input::KeyCode::*;
use self::tcod::console::Root;
use ::actors::Object;
use ::map::data::Map;

pub enum PlayerAction {
    TookTurn,
    Moved(MoveCommand),
    DidntTakeTurn,
    Exit,
}

pub struct MoveCommand {
    dx: i32,
    dy: i32
}
impl MoveCommand {
    pub fn new(dx: i32, dy: i32) -> MoveCommand {
        MoveCommand {
            dx, dy
        }
    }
    pub fn execute(self, entity: &mut Object, map: &Map) {
        if !map[(entity.x + self.dx) as usize][(entity.y + self.dy) as usize].blocked {
            entity.move_by(self.dx, self.dy)
        }
    }
}

pub fn handle_keys(root: &mut Root) -> PlayerAction {
    let key = root.wait_for_keypress(true);

    match key {
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
            PlayerAction::DidntTakeTurn
        },
        Key { code: Escape, .. } => {
            PlayerAction::Exit
        },
        Key { code: Up, .. } => {
            PlayerAction::Moved(MoveCommand::new(0, -1))
        }
        Key { code: Down, .. } => {
            PlayerAction::Moved(MoveCommand::new(0, 1))
        }
        Key { code: Left, .. } => {
            PlayerAction::Moved(MoveCommand::new(-1, 0))
        }
        Key { code: Right, .. } => {
            PlayerAction::Moved(MoveCommand::new(1, 0))
        }
        _ => {
            PlayerAction::DidntTakeTurn
        }
    }
}