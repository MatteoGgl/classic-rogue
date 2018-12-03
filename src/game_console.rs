extern crate tcod;

use self::tcod::console::*;
use self::tcod::Console;
use self::tcod::colors::Color;

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const CONSOLE_FONT: &str = "terminal10x10_gs_tc.png";

pub struct RootConsole {
    pub console: Root,
}
impl RootConsole {
    pub fn new() -> RootConsole {
        let root_console = Root::initializer()
            .font(CONSOLE_FONT, FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Rust/libtcod tutorial")
            .init();

        RootConsole {
            console: root_console
        }
    }
    pub fn blit_map(&mut self, map_console: &mut Offscreen, width: i32, height: i32) {
        blit(map_console, (0, 0), (width, height), &mut self.console, (0, 0), 1.0, 1.0);
    }
}

pub struct MapConsole {
    pub console: Offscreen,
}
impl MapConsole {
    pub fn new(width: i32, height: i32) -> MapConsole {
        MapConsole {
            console: Offscreen::new(width, height),
        }
    }
    pub fn draw_bg(&mut self, x: i32, y: i32, color: Color) {
        self.console.set_char_background(x, y, color, BackgroundFlag::Set);
    }
    pub fn draw_fg(&mut self, x: i32, y: i32, char: char, color: Color) {
        self.console.put_char(x, y, char, BackgroundFlag::Set);
    }
}
