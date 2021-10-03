//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

mod chess_server;
mod render;

use std::process::exit;
use std::sync::mpsc::{self, Receiver, Sender};
use std::{env, path};

use chess_engine::chess_game::{BoardMove, BoardPosition, ChessPieceColor, ChessPieceId, Game};
use ggez::event;
use ggez::event::MouseButton;
use ggez::graphics::{self, Font, PxScale};
use ggez::{Context, GameResult};

use glam::Vec2;
use render::*;

struct SpriteSheet {
    pawn_white: graphics::Image,
    pawn_black: graphics::Image,
    bishop_white: graphics::Image,
    bishop_black: graphics::Image,
    bishop_white_on_black_square: graphics::Image,
    bishop_black_on_black_square: graphics::Image,
    knight_white: graphics::Image,
    knight_black: graphics::Image,
    rook_white: graphics::Image,
    rook_black: graphics::Image,
    queen_white: graphics::Image,
    queen_black: graphics::Image,
    king_white: graphics::Image,
    king_black: graphics::Image,
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

struct Icons {
    //surrender: graphics::Image,
    replay: graphics::Image,
    settings: graphics::Image,
    //arrow_back: graphics::Image,
    exit: graphics::Image,
    confirm: graphics::Image,
}

#[derive(Clone, Copy)]
enum Action {
    StartServer,
    StartClient,
    Restart,
    Quit,
    None,
}

struct PendingAction {
    text: String,
    confirm: graphics::Image,
    cancel: graphics::Image,
    confirm_value: Action,
    cancel_value: Action,
}

struct RenderConfig {
    spritesets: Vec<SpriteSheet>,
    fontsets: Vec<FontSet>,

    // because every frame is redrawn in a render loop
    // you can switch render or font at any time
    active_sprites_index: usize,
    active_fontset_index: usize,

    icons: Icons,
}

struct InputStatus {
    pos_x: f32,
    pos_y: f32,
    mouse_down: bool,
    mouse_clicked: bool,
    mouse_released: bool,
}

pub struct MainState {
    frame: u64,
    server: chess_server::Server,
    render_config: RenderConfig,
    active_message: Option<PendingAction>,
    active_game: ActiveGame,
    input_staus: InputStatus,
}

macro_rules! add_piece_sprite {
    ($ctx:expr,$path:expr, $name:expr) => {{
        graphics::Image::new($ctx, concat!("/piece/", $path, "/", $name, ".png")).unwrap()
    }};
}

macro_rules! add_png {
    ($ctx:expr, $path:expr) => {{
        graphics::Image::new($ctx, concat!("/img/", $path, ".png")).unwrap()
    }};
}

macro_rules! add_font {
    ($ctx:expr, $path:expr) => {{
        graphics::Font::new($ctx, concat!("/font/", $path, ".ttf")).unwrap()
    }};
}

macro_rules! add_sprite_sheet {
    ($ctx:expr, $path:expr) => {{
        SpriteSheet {
            pawn_white: add_piece_sprite!($ctx, $path, "wP"),
            pawn_black: add_piece_sprite!($ctx, $path, "bP"),
            bishop_white: add_piece_sprite!($ctx, $path, "wB"),
            bishop_black: add_piece_sprite!($ctx, $path, "bB"),
            bishop_white_on_black_square: add_piece_sprite!($ctx, $path, "wB"),
            bishop_black_on_black_square: add_piece_sprite!($ctx, $path, "bB"),
            knight_white: add_piece_sprite!($ctx, $path, "wN"),
            knight_black: add_piece_sprite!($ctx, $path, "bN"),
            rook_white: add_piece_sprite!($ctx, $path, "wR"),
            rook_black: add_piece_sprite!($ctx, $path, "bR"),
            queen_white: add_piece_sprite!($ctx, $path, "wQ"),
            queen_black: add_piece_sprite!($ctx, $path, "bQ"),
            king_white: add_piece_sprite!($ctx, $path, "wK"),
            king_black: add_piece_sprite!($ctx, $path, "bK"),
        }
    }};
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // init sprites and fonts

        let regular_sprites = add_sprite_sheet!(ctx, "regular");
        let horsey_sprites = add_sprite_sheet!(ctx, "horsey");
        let mut emoji_sprites = add_sprite_sheet!(ctx, "emoji");

        emoji_sprites.bishop_black_on_black_square = add_piece_sprite!(ctx, "emoji", "bB2");
        emoji_sprites.bishop_white_on_black_square = add_piece_sprite!(ctx, "emoji", "wB2");

        let regular_font = FontSet {
            font: Font::default(),
            font_size: PxScale { x: 30f32, y: 30f32 },
        };

        let nice_font = FontSet {
            font: add_font!(ctx, "NotoSans-Bold"),
            font_size: PxScale { x: 30f32, y: 30f32 },
        };

        let icons = Icons {
            //surrender: addpng!(ctx, "surrender"),
            replay: add_png!(ctx, "replay"),
            settings: add_png!(ctx, "settings"),
            //arrow_back: addpng!(ctx, "arrow_back"),
            exit: add_png!(ctx, "exit"),
            confirm: add_png!(ctx, "confirm"),
        };

        let mut game = chess_engine::chess_game::Game::new();
        game.set_up_board();

        let message = Some(PendingAction {
            text: "Sluta spela?".to_string(),
            confirm: icons.confirm.clone(),
            cancel: icons.exit.clone(),
            confirm_value: Action::StartServer,
            cancel_value: Action::None,
        });

        let s = MainState {
            server: chess_server::start_server(),
            frame: 0,
            render_config: RenderConfig {
                spritesets: vec![regular_sprites, horsey_sprites, emoji_sprites],
                fontsets: vec![regular_font, nice_font],
                active_fontset_index: 1,
                active_sprites_index: 0,
                icons,
            },
            active_game: ActiveGame {
                game,
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
            active_message: message,
        };
        Ok(s)
    }
}

fn get_possible_moves_from_position(game: Game, pos: BoardPosition) -> Vec<BoardPosition> {
    let mut possible_moves: Vec<BoardPosition> = Vec::new();

    for y in 0..8 {
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

/** Handle user input logic to move pieces */
fn do_game_logic(main_state: &mut MainState) {
    let input = &main_state.input_staus;
    let state = &mut main_state.active_game;

    // select a square and make hover
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

            //check if move is valid, as all the moves have already been checked a simple contains marks it as valid
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
                    // this should be impossible to achive
                    println!("Bruh how?")
                } else {
                    if state.game.game_is_over() {
                        // updates the popup message asking to play again
                        let winner = state.game.get_winner();
                        let text = match winner {
                            None => "Oavgjort, Spela igen?".to_string(),
                            Some(ChessPieceColor::White) => "Vit vann, Spela igen?".to_string(),
                            Some(ChessPieceColor::Black) => "Svart vann, Spela igen?".to_string(),
                        };

                        main_state.input_staus.mouse_released = false;
                        main_state.active_message = Some(PendingAction {
                            text,
                            confirm: main_state.render_config.icons.confirm.clone(),
                            cancel: main_state.render_config.icons.exit.clone(),
                            confirm_value: Action::Restart,
                            cancel_value: Action::None,
                        })
                    }
                }
            }
        } else {
            state.possible_moves = None;
            state.hover_position = None;
        }
    }
}

fn network_callback(input: String) -> String {
    println!("GOT::: {}", input);
    return "hello from server".to_string();
}

/** Handle action triggered by a popup */
fn handle_action(action: Action, state: &mut MainState) {
    

    match action {
        Action::StartClient => {}
        Action::StartServer => {
            
            /*thread::spawn(|| {
                let server_result = chess_server::start(state, network_callback);
                if server_result.is_err() {
                    println!("Server Error");
                }
            })*/
        }
        Action::Restart => {
            let mut game = chess_engine::chess_game::Game::new();
            game.set_up_board();
            state.active_game.game = game;
            state.active_game.hover_position = None;
            state.active_game.possible_moves = None;
            state.active_game.selected_square = None;
        }
        Action::Quit => exit(0),
        Action::None => {}
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        chess_server::server_loop(&mut self.server, &mut self.active_game.game);

        // cant move anything while a popup is active, game is paused
        if self.active_message.is_none() {
            do_game_logic(self);
        }

        // render board
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
        if self.active_message.is_none() {
            let selected_button = render_buttons(ctx, self);

            // handle main input buttons
            if selected_button.is_some() && self.input_staus.mouse_released {
                match selected_button.unwrap() {
                    0 => {
                        self.active_message = Some(PendingAction {
                            text: "Sluta spela?".to_string(),
                            confirm: self.render_config.icons.confirm.clone(),
                            cancel: self.render_config.icons.exit.clone(),
                            confirm_value: Action::Quit,
                            cancel_value: Action::None,
                        })
                    }
                    1 => {
                        self.active_message = Some(PendingAction {
                            text: "Spela igen?".to_string(),
                            confirm: self.render_config.icons.confirm.clone(),
                            cancel: self.render_config.icons.exit.clone(),
                            confirm_value: Action::Restart,
                            cancel_value: Action::None,
                        })
                    }
                    2 => {
                        self.render_config.active_sprites_index += 1;
                        if self.render_config.active_sprites_index
                            >= self.render_config.spritesets.len()
                        {
                            self.render_config.active_sprites_index = 0
                        }
                    }
                    _ => {}
                }
            }
        } else {
            // waiting for the user to release click on a message button
            // be aware that clicking anywhere on the screen counts as Action::None
            let action = render_message(ctx, self)?;
            if self.input_staus.mouse_released {
                handle_action(action, self);
                self.active_message = None;
            }
        }

        // mouse_released and mouse_clicked is only active for 1 frame
        self.input_staus.mouse_released = false;
        self.input_staus.mouse_clicked = false;
        //let fps = ggez::timer::fps(ctx);
        //println!("{}",fps);
        self.frame += 1;
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
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        self.input_staus.pos_x = x;
        self.input_staus.pos_y = y;
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
        /*println!(
            "Mouse motion, x: {}, y: {}, relative x: {}, relative y: {}",
            x, y, xrel, yrel
        );*/
    }
}

pub fn main() -> GameResult {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    
    // set up resource path from the base of the project
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("res");
        path
    } else {
        path::PathBuf::from("./res")
    };

    let cb =
        ggez::ContextBuilder::new("vinlag_vicil_chess", "Vincent Lagerros and Victor Millberg")
            .window_setup(
                ggez::conf::WindowSetup::default()
                    .title("Schack Deluxe Edition")
                    .icon("/piece/horsey/wN.png")
                    .samples(ggez::conf::NumSamples::One),
            )
            .window_mode(
                ggez::conf::WindowMode::default()
                    .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1)
                    .borderless(false),
            )
            .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state);
    /**/
}
