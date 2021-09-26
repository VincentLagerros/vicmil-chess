use chess_engine::chess_game::{ChessPiece, ChessPieceColor, ChessPieceId};
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, GameResult};
use glam::*;

use crate::{MainState, SpriteSheet};

pub(crate) const SCREEN_SIZE: (f32, f32) = (720f32, 720f32);

const BOARD_SIZE: usize = 8;
const BOARD_RENDER_SIZE: f32 = 720f32;
const BOARD_RENDER_TILE_SIZE: f32 = BOARD_RENDER_SIZE / BOARD_SIZE as f32;

const BOARD_RENDER_START: (f32, f32) = (
    SCREEN_SIZE.0 / 2.0 - BOARD_RENDER_SIZE / 2.0,
    SCREEN_SIZE.1 / 2.0 - BOARD_RENDER_SIZE / 2.0,
);

const BLACK_BOARD_COLOR: Color = Color {
    r: 0.4367,
    g: 0.31,
    b: 0.2533,
    a: 1.0,
};

const WHITE_BOARD_COLOR: Color = Color {
    r: 0.6591,
    g: 0.5125,
    b: 0.4284,
    a: 1.0,
};

const BACKGROUND_COLOR: Color = Color {
    r: 0.19,
    g: 0.18,
    b: 0.17,
    a: 1.0,
};

fn get_piece_image(id : ChessPieceId, color : ChessPieceColor, square_is_dark : bool, sprites : &SpriteSheet) -> &graphics::Image {
    if color == ChessPieceColor::White {
        if id == ChessPieceId::Bishop && square_is_dark {
            return &sprites.bishop_light_on_dark_square;
        }
        return match id {
            ChessPieceId::Pawn => &sprites.pawn_light,
            ChessPieceId::Knight => &sprites.knight_light,
            ChessPieceId::Rook => &sprites.rook_light,
            ChessPieceId::King => &sprites.king_light,
            ChessPieceId::Queen => &sprites.queen_light,
            ChessPieceId::Bishop => &sprites.bishop_light,
        }
    } else {
        if id == ChessPieceId::Bishop && square_is_dark {
            return &sprites.bishop_dark_on_dark_square;
        }
        return match id {
            ChessPieceId::Pawn => &sprites.pawn_dark,
            ChessPieceId::Knight => &sprites.knight_dark,
            ChessPieceId::Rook => &sprites.rook_dark,
            ChessPieceId::King => &sprites.king_dark,
            ChessPieceId::Queen => &sprites.queen_dark,
            ChessPieceId::Bishop => &sprites.bishop_dark,
        }
    }
   
}

pub(crate) fn render_clear(ctx: &mut Context) {
    graphics::clear(ctx, BACKGROUND_COLOR);
}

/** Just renders the background board */
pub(crate) fn render_board(ctx: &mut Context) -> GameResult<()> {
    let bg_square = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        Rect::new(0.0, 0.0, BOARD_RENDER_SIZE, BOARD_RENDER_SIZE),
        WHITE_BOARD_COLOR,
    )?;
    graphics::draw(
        ctx,
        &bg_square,
        (Vec2::new(BOARD_RENDER_START.0, BOARD_RENDER_START.1),),
    )?;

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if (x + y) % 2 == 0 {
                continue;
            }

            let square = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0.0, 0.0, BOARD_RENDER_TILE_SIZE, BOARD_RENDER_TILE_SIZE),
                BLACK_BOARD_COLOR,
            )?;
            graphics::draw(
                ctx,
                &square,
                (Vec2::new(
                    BOARD_RENDER_START.0 + (x as f32) * BOARD_RENDER_TILE_SIZE,
                    BOARD_RENDER_START.1 + (y as f32) * BOARD_RENDER_TILE_SIZE,
                ),),
            )?;
        }
    }

    Ok(())
}

pub(crate) fn render_pieces(ctx: &mut Context, state: &MainState) -> GameResult<()> {
    let game = match state.game {
        Some(g) => g,
        None => return Err(ggez::GameError::CustomError("game board not found".to_string())),
    };

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE { 
            let piece = match game.board[x + y*BOARD_SIZE] {
                Some(p) => p,
                None => continue,  
            };

            let dist = Vec2::new(
                BOARD_RENDER_START.0 + (x as f32) * BOARD_RENDER_TILE_SIZE,
                BOARD_RENDER_START.1 + (y as f32) * BOARD_RENDER_TILE_SIZE,
            );

            let is_on_white = (x + y) % 2 == 1;

            graphics::draw(
                ctx,
                get_piece_image(piece.id,piece.color,is_on_white,&state.active_sprites),
                graphics::DrawParam::new()
                .dest(dist)
                .offset(state.active_sprites.offset)
            )?;
        }
    }

    Ok(())
}
