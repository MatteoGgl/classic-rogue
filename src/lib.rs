extern crate tcod;

use std::error::Error;
use tcod::Console;
use game_console::*;

mod game_console;
mod input;
mod map;
mod actors;

const LIMIT_FPS: i32 = 20;

pub fn init_game() -> Result<(), Box<dyn Error>> {
    use input::PlayerAction;
    let mut root = RootConsole::new();
    tcod::system::set_fps(LIMIT_FPS);
    
    let dungeon = map::generate_dungeon();
    let mut objects: Vec<actors::Object> = vec![];
    actors::create_player(&mut objects, dungeon.player_start);

    let mut map = MapConsole::new(dungeon.width, dungeon.height);

    //Main game loop
    while !root.console.window_closed() {
        for y in 0..dungeon.height {
            for x in 0..dungeon.width {
                map.draw_fg(x, y, ' ', tcod::colors::WHITE);
                map.draw_bg(x, y, dungeon.map[x as usize][y as usize].background);
            }
        }
        objects.iter().for_each(|obj|{
            map.draw_fg(obj.x, obj.y, obj.char, obj.color);
        });
        root.blit_map(&mut map.console, dungeon.width, dungeon.height);
        root.console.flush();

        match input::handle_keys(&mut root.console) {
            PlayerAction::Moved(command) => {
                command.execute(&mut objects[0], &dungeon.map);
            }
            PlayerAction::Exit => break,
            PlayerAction::TookTurn => continue,
            PlayerAction::DidntTakeTurn => continue
        }
    }

    Ok(())
}
