//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

mod render;

use std::{env, path};

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use glam::Vec2;
use render::*;

struct SpriteSheet {
    pawn_light: graphics::Image,
    pawn_dark: graphics::Image,
    bishop_light: graphics::Image,
    bishop_dark: graphics::Image,
    bishop_light_on_dark_square: graphics::Image,
    bishop_dark_on_dark_square: graphics::Image,
    knight_light: graphics::Image,
    knight_dark: graphics::Image,
    rook_light: graphics::Image,
    rook_dark: graphics::Image,
    queen_light: graphics::Image,
    queen_dark: graphics::Image,
    king_light: graphics::Image,
    king_dark: graphics::Image,
    offset : Vec2,
}

struct MainState {
    active_sprites: SpriteSheet,
    game : Option<chess_engine::chess_game::Game>
}

macro_rules! addpng {
    ($ctx:expr, $path:expr) => {{
        graphics::Image::new($ctx, concat!("/png90/", $path, ".png")).unwrap()
    }};
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let active_sprites = SpriteSheet {
            pawn_light: addpng!(ctx, "pl"),
            pawn_dark: addpng!(ctx, "pd"),
            bishop_light: addpng!(ctx, "bl"),
            bishop_dark: addpng!(ctx, "bd"),
            bishop_light_on_dark_square: addpng!(ctx, "bl"),
            bishop_dark_on_dark_square: addpng!(ctx, "bd"),
            knight_light: addpng!(ctx, "nl"),
            knight_dark: addpng!(ctx, "nd"),
            rook_light: addpng!(ctx, "rl"),
            rook_dark: addpng!(ctx, "rd"),
            queen_light: addpng!(ctx, "ql"),
            queen_dark: addpng!(ctx, "qd"),
            king_light: addpng!(ctx, "kl"),
            king_dark: addpng!(ctx, "kd"),
            offset: Vec2::new(0.0,0.0)
        };

        /*let active_sprites = SpriteSheet {
            pawn_light: addpng!(ctx, "mpl"),
            pawn_dark: addpng!(ctx, "mpd"),
            bishop_light: addpng!(ctx, "mbl_2"),
            bishop_dark: addpng!(ctx, "mbd"),
            bishop_light_on_dark_square: addpng!(ctx, "mbl"),
            bishop_dark_on_dark_square: addpng!(ctx, "mbd_2"),
            knight_light: addpng!(ctx, "mnl"),
            knight_dark: addpng!(ctx, "mnd"),
            rook_light: addpng!(ctx, "mrl"),
            rook_dark: addpng!(ctx, "mrd"),
            queen_light: addpng!(ctx, "mql"),
            queen_dark: addpng!(ctx, "mqd"),
            king_light: addpng!(ctx, "mkl"),
            king_dark: addpng!(ctx, "mkd"),
            offset: Vec2::new(-0.05,-0.05)
        };*/

        let mut game = chess_engine::chess_game::Game::new();
        game.set_up_board();

        let s = MainState {
            active_sprites,
            game: Some(game),
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.game.is_some() { // game exists
            render_clear(ctx);
            render_board(ctx)?;
            render_pieces(ctx, &self)?;
        }
        
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("res");
        path
    } else {
        path::PathBuf::from("./res")
    };

    let cb =
        ggez::ContextBuilder::new("vinlag_vicil_chess", "Vincent Lagerros and Victor Millberg")
            .window_setup(ggez::conf::WindowSetup::default().title("Chess!"))
            .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
            .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
