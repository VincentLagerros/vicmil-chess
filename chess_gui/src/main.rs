//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

mod render;

use std::{env, path};

use chess_engine::chess_game::{BoardMove, BoardPosition, ChessPieceId, Game};
use ggez::event;
use ggez::event::MouseButton;
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
    offset: Vec2,
}

struct MainState {
    active_sprites: SpriteSheet,
    game: chess_engine::chess_game::Game,
    selected_square: Option<BoardPosition>,
    hover_position: Option<Vec2>,
    possible_moves: Option<Vec<BoardPosition>>,

    pos_x: f32,
    pos_y: f32,
    mouse_down: bool,
    mouse_clicked: bool,
    mouse_released: bool,
}

macro_rules! addpng {
    ($ctx:expr, $path:expr) => {{
        graphics::Image::new($ctx, concat!("/png90/", $path, ".png")).unwrap()
    }};
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        /*let active_sprites = SpriteSheet {
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
            offset: Vec2::new(0.0, 0.0),
        };*/

        let active_sprites = SpriteSheet {
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
        };

        let mut game = chess_engine::chess_game::Game::new();
        game.set_up_board();

        let s = MainState {
            active_sprites,
            game: game,
            selected_square: None,
            hover_position: None,
            pos_x: 0.0,
            pos_y: 0.0,
            mouse_down: false,
            mouse_clicked: false,
            possible_moves: None,
            mouse_released: false,
        };
        Ok(s)
    }
}

fn get_possible_moves_from_position(game: Game, pos: BoardPosition) -> Vec<BoardPosition> {
    let mut possible_moves: Vec<BoardPosition> = Vec::new();

    for y in 0..8 {
        print!("{} ", 8 - y);
        for x in 0..8 {
            let board_move;
            board_move = BoardMove::new(pos.x, pos.y, x, y);

            let mut board_copy = game.clone();
            if board_copy
                .move_piece(board_move, true, Some(ChessPieceId::Queen))
                .is_ok()
            {
                // Color square red if piece can move there
                possible_moves.push(BoardPosition::new(x, y))
            }
        }
    }

    return possible_moves;
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // game exists, aka not in menu

        if self.mouse_down {
            let mouse_pos = Vec2::new(self.pos_x, self.pos_y);
            self.hover_position = Some(mouse_pos);
            if self.mouse_clicked {
                let selected_square = get_square_from_screen(mouse_pos);

                if selected_square.is_some()
                    && self
                        .game
                        .get_board_piece_clone(selected_square.unwrap())
                        .is_some()
                {
                    self.selected_square = selected_square;
                    self.possible_moves = Some(get_possible_moves_from_position(
                        self.game,
                        selected_square.unwrap(),
                    ));
                } else {
                    self.selected_square = None;
                }
            }
        } else {
            if self.mouse_released && self.hover_position.is_some() {
                let move_square = get_square_from_screen(self.hover_position.unwrap());
                if move_square.is_some()
                    && self.selected_square.is_some()
                    && self.possible_moves.is_some()
                    && self
                        .possible_moves
                        .as_ref()
                        .unwrap()
                        .contains(&move_square.unwrap())
                {
                    let move_to = move_square.unwrap();
                    let move_from = self.selected_square.unwrap();

                    let result = self.game.move_piece(
                        BoardMove::new(move_from.x, move_from.y, move_to.x, move_to.y),
                        false,
                        Some(ChessPieceId::Queen),
                    );
                    if result.is_err() {
                        println!("Bruh how?")
                    }
                }
            } else {
                self.possible_moves = None;
                self.hover_position = None;
            }
        }
            render_clear(ctx);
            render_board(ctx)?;
            render_highlight(ctx, self.selected_square, HIGHLIGHT_COLOR)?;
            if self.possible_moves.is_some() {
                for pos in self.possible_moves.as_ref().unwrap() {
                    render_highlight(ctx, Some(*pos), MOVE_COLOR)?;
                }
            }

            render_pieces(ctx, self)?;
        

        self.mouse_released = false;
        self.mouse_clicked = false;
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.mouse_down = true;
        self.mouse_clicked = true;
        //println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.mouse_down = false;
        self.mouse_released = true;
        //println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        self.pos_x = x;
        self.pos_y = y;
        //if self.mouse_down {
        // Mouse coordinates are PHYSICAL coordinates, but here we want logical coordinates.

        // If you simply use the initial coordinate system, then physical and logical
        // coordinates are identical.

        // If you change your screen coordinate system you need to calculate the
        // logical coordinates like this:
        /*
        let screen_rect = graphics::screen_coordinates(_ctx);
        let size = graphics::window(_ctx).inner_size();
        self.pos_x = (x / (size.width  as f32)) * screen_rect.w + screen_rect.x;
        self.pos_y = (y / (size.height as f32)) * screen_rect.h + screen_rect.y;
        */
        //}
        /*println!(
            "Mouse motion, x: {}, y: {}, relative x: {}, relative y: {}",
            x, y, xrel, yrel
        );*/
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
