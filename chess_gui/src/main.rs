//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

mod render;

use std::{env, path};

use chess_engine::chess_game::{BoardMove, BoardPosition, ChessPieceId, Game};
use ggez::event;
use ggez::event::MouseButton;
use ggez::graphics::{self, Font, PxScale};
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

struct FontSet {
    font: Font,
    font_size: PxScale,
}

struct ActiveGame {
    game: chess_engine::chess_game::Game,
    selected_square: Option<BoardPosition>,
    hover_position: Option<Vec2>,
    possible_moves: Option<Vec<BoardPosition>>,
}

struct RenderConfig {
    spritesets: Vec<SpriteSheet>,
    fontsets: Vec<FontSet>,

    active_sprites_index: usize,
    active_fontset_index: usize,
}

struct InputStatus {
    pos_x: f32,
    pos_y: f32,
    mouse_down: bool,
    mouse_clicked: bool,
    mouse_released: bool,
}

struct MainState {
    render_config: RenderConfig,
    active_game: ActiveGame,
    input_staus: InputStatus,
}

macro_rules! add_piece_sprite {
    ($ctx:expr,$path:expr, $name:expr) => {{
        graphics::Image::new($ctx, concat!("/piece/", $path, "/", $name, ".png")).unwrap()
    }};
}

macro_rules! addpng {
    ($ctx:expr, $path:expr) => {{
        graphics::Image::new($ctx, concat!("/img/", $path, ".png")).unwrap()
    }};
}

macro_rules! addfont {
    ($ctx:expr, $path:expr) => {{
        graphics::Font::new($ctx, concat!("/font/", $path, ".ttf")).unwrap()
    }};
}
macro_rules! add_sprite_sheet {
    ($ctx:expr, $path:expr, $offset:expr) => {{
        SpriteSheet {
            pawn_light: add_piece_sprite!($ctx, $path, "wP"),
            pawn_dark: add_piece_sprite!($ctx, $path, "bP"),
            bishop_light: add_piece_sprite!($ctx, $path, "wB"),
            bishop_dark: add_piece_sprite!($ctx, $path, "bB"),
            bishop_light_on_dark_square: add_piece_sprite!($ctx, $path, "wB"),
            bishop_dark_on_dark_square: add_piece_sprite!($ctx, $path, "bB"),
            knight_light: add_piece_sprite!($ctx, $path, "wN"),
            knight_dark: add_piece_sprite!($ctx, $path, "bN"),
            rook_light: add_piece_sprite!($ctx, $path, "wR"),
            rook_dark: add_piece_sprite!($ctx, $path, "bR"),
            queen_light: add_piece_sprite!($ctx, $path, "wQ"),
            queen_dark: add_piece_sprite!($ctx, $path, "bQ"),
            king_light: add_piece_sprite!($ctx, $path, "wK"),
            king_dark: add_piece_sprite!($ctx, $path, "bK"),
            offset: $offset,
        }
    }};
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let regular_sprites = add_sprite_sheet!(ctx, "regular",Vec2::new(0.0,0.0));
        let horsey_sprites = add_sprite_sheet!(ctx, "horsey",Vec2::new(5.0, 5.0));
        let meme_sprites = SpriteSheet {
            pawn_light: add_piece_sprite!(ctx, "meme", "mpl"),
            pawn_dark: add_piece_sprite!(ctx, "meme", "mpd"),
            bishop_light: add_piece_sprite!(ctx, "meme", "mbl_2"),
            bishop_dark: add_piece_sprite!(ctx, "meme", "mbd"),
            bishop_light_on_dark_square: add_piece_sprite!(ctx, "meme", "mbl"),
            bishop_dark_on_dark_square: add_piece_sprite!(ctx, "meme", "mbd_2"),
            knight_light: add_piece_sprite!(ctx, "meme", "mnl"),
            knight_dark: add_piece_sprite!(ctx, "meme", "mnd"),
            rook_light: add_piece_sprite!(ctx, "meme", "mrl"),
            rook_dark: add_piece_sprite!(ctx, "meme", "mrd"),
            queen_light: add_piece_sprite!(ctx, "meme", "mql"),
            queen_dark: add_piece_sprite!(ctx, "meme", "mqd"),
            king_light: add_piece_sprite!(ctx, "meme", "mkl"),
            king_dark: add_piece_sprite!(ctx, "meme", "mkd"),
            offset: Vec2::new(5.0, 5.0),
        };

        let regular_font = FontSet {
            font: Font::default(),
            font_size: PxScale { x: 30f32, y: 30f32 },
        };

        let nice_font = FontSet {
            font: addfont!(ctx, "NotoSans-Bold"),
            font_size: PxScale { x: 30f32, y: 30f32 },
        };

        let mut game = chess_engine::chess_game::Game::new();
        game.set_up_board();

        let s = MainState {
            render_config: RenderConfig {
                spritesets: vec![regular_sprites, meme_sprites, horsey_sprites],
                fontsets: vec![regular_font, nice_font],
                active_fontset_index: 1,
                active_sprites_index: 2,
            },
            active_game: ActiveGame {
                game: game,
                selected_square: None,
                hover_position: None,
                possible_moves: None,
            },
            input_staus: InputStatus {
                pos_x: 0.0,
                pos_y: 0.0,
                mouse_down: false,
                mouse_clicked: false,
                mouse_released: false,
            },
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

fn do_game_logic(input: &InputStatus, state: &mut ActiveGame) {
    if input.mouse_down {
        let mouse_pos = Vec2::new(input.pos_x, input.pos_y);
        state.hover_position = Some(mouse_pos);
        if input.mouse_clicked {
            let selected_square = get_square_from_screen(mouse_pos);

            if selected_square.is_some()
                && state
                    .game
                    .get_board_piece_clone(selected_square.unwrap())
                    .is_some()
            {
                state.selected_square = selected_square;
                state.possible_moves = Some(get_possible_moves_from_position(
                    state.game,
                    selected_square.unwrap(),
                ));
            } else {
                state.selected_square = None;
            }
        }
    } else {
        if input.mouse_released && state.hover_position.is_some() {
            let move_square = get_square_from_screen(state.hover_position.unwrap());
            if move_square.is_some()
                && state.selected_square.is_some()
                && state.possible_moves.is_some()
                && state
                    .possible_moves
                    .as_ref()
                    .unwrap()
                    .contains(&move_square.unwrap())
            {
                let move_to = move_square.unwrap();
                let move_from = state.selected_square.unwrap();

                let result = state.game.move_piece(
                    BoardMove::new(move_from.x, move_from.y, move_to.x, move_to.y),
                    false,
                    Some(ChessPieceId::Queen),
                );
                if result.is_err() {
                    println!("Bruh how?")
                }
            }
        } else {
            state.possible_moves = None;
            state.hover_position = None;
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        do_game_logic(&self.input_staus, &mut self.active_game);

        render_clear(ctx);
        render_board(ctx)?;
        render_numbers(ctx, &self.render_config)?;
        render_highlight(ctx, self.active_game.selected_square, HIGHLIGHT_COLOR)?;
        if self.active_game.possible_moves.is_some() {
            for pos in self.active_game.possible_moves.as_ref().unwrap() {
                render_highlight(ctx, Some(*pos), MOVE_COLOR)?;
            }
        }

        render_pieces(ctx, &self.render_config, &mut self.active_game)?;

        self.input_staus.mouse_released = false;
        self.input_staus.mouse_clicked = false;
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
        self.input_staus.mouse_down = true;
        self.input_staus.mouse_clicked = true;
        //println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.input_staus.mouse_down = false;
        self.input_staus.mouse_released = true;
        //println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        self.input_staus.pos_x = x;
        self.input_staus.pos_y = y;
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
