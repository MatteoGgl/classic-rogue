extern crate rand;

use map::data::*;
use self::rand::Rng;

pub mod data;

pub fn generate_dungeon() -> Dungeon {
    let mut dungeon = Dungeon::new();

    create_rooms(&mut dungeon);
    //place_objects();

    dungeon
}

fn create_rooms(dungeon : &mut Dungeon) {
    for _ in 0..MAX_ROOMS {
        // random width and height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        // random position without going out of the boundaries of the map
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        let new_room = Rect::new(x, y, w, h);

        let failed = dungeon.rooms.iter().any(|other_room| new_room.intersects_with(other_room));

        if !failed {
            dungeon.create_room(new_room);
            if dungeon.rooms.is_empty() {
                dungeon.player_start = new_room.center();
            } else {
                let (prev_x, prev_y) = dungeon.rooms[dungeon.rooms.len() - 1].center();
                let (center_x, center_y) = new_room.center();
                if rand::random() {
                    dungeon.create_h_tunnel(prev_x, center_x, prev_y);
                    dungeon.create_v_tunnel(prev_y, center_y, center_x);
                } else {
                    dungeon.create_v_tunnel(prev_y, center_y, prev_x);
                    dungeon.create_h_tunnel(prev_x, center_x, center_y);
                }
            }

            dungeon.rooms.push(new_room);
        };
    }
}
