use chess_engine::chess_game::{BoardPosition, ChessPiece, ChessPieceColor, ChessPieceId};
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, GameResult};
use glam::*;

use crate::{ActiveGame, RenderConfig, SpriteSheet};

pub(crate) const SCREEN_SIZE: (f32, f32) = (820f32, 820f32);

const BOARD_SIZE: u8 = 8;
const BOARD_NUMBERS : [char; BOARD_SIZE as usize] = [
    '8',
    '7',
    '6',
    '5',
    '4',
    '3',
    '2',
    '1',
];
const BOARD_LETTERS : [char; BOARD_SIZE as usize] = [
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
];
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

pub const HIGHLIGHT_COLOR: Color = Color {
    r: 0.49,
    g: 0.88,
    b: 0.47,
    a: 0.5,
};

pub const MOVE_COLOR: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 0.5,
};


const BACKGROUND_COLOR: Color = Color {
    r: 0.19,
    g: 0.18,
    b: 0.17,
    a: 1.0,
};

pub fn get_square_from_screen(mouse : Vec2) -> Option<BoardPosition>{
    // because of margin this has to take place
    let zero_offset = mouse - (Vec2::new(SCREEN_SIZE.0 - BOARD_RENDER_SIZE, SCREEN_SIZE.1 - BOARD_RENDER_SIZE) / 2.0);
    if zero_offset.x < 0.0 || zero_offset.x > BOARD_RENDER_SIZE || zero_offset.y < 0.0 ||zero_offset.y > BOARD_RENDER_SIZE {
        return None;
    }
    
    return Some(BoardPosition::new((zero_offset.x/BOARD_RENDER_TILE_SIZE) as u8, (zero_offset.y/BOARD_RENDER_TILE_SIZE) as u8));
}

fn get_piece_image(
    id: ChessPieceId,
    color: ChessPieceColor,
    square_is_dark: bool,
    sprites: &SpriteSheet,
) -> &graphics::Image {
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
        };
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
        };
    }
}

fn get_render_pos(x: u8, y: u8) -> Vec2 {
    Vec2::new(
        BOARD_RENDER_START.0 + (x as f32) * BOARD_RENDER_TILE_SIZE,
        BOARD_RENDER_START.1 + (y as f32) * BOARD_RENDER_TILE_SIZE,
    )
}

pub(crate) fn render_clear(ctx: &mut Context) {
    graphics::clear(ctx, BACKGROUND_COLOR);
}

pub(crate) fn render_highlight(ctx: &mut Context, pos: Option<BoardPosition>, color : Color) -> GameResult<()> {
    let safe_pos = match pos {
        Some(p) => p,
        None => return Ok(()),
    };

    let square = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        Rect::new(0.0, 0.0, BOARD_RENDER_TILE_SIZE, BOARD_RENDER_TILE_SIZE),
        color,
    )?;
    graphics::draw(ctx, &square, (get_render_pos(safe_pos.x, safe_pos.y),))?;

    Ok(())
}

fn get_color(square_is_dark : bool) -> Color {
    return if square_is_dark { BLACK_BOARD_COLOR } else { WHITE_BOARD_COLOR };
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
            graphics::draw(ctx, &square, (get_render_pos(x, y),))?;
        }
    }

    Ok(())
}

pub(crate) fn render_numbers(ctx: &mut Context, config: & RenderConfig)-> GameResult<()> {
    let active_font = &config.fontsets[config.active_fontset_index];
    let x_add_offset = active_font.font_size.x / 4.0;
    let y_offset = Vec2::new(x_add_offset, BOARD_RENDER_TILE_SIZE - active_font.font_size.y);
    let x_offset = Vec2::new(BOARD_RENDER_TILE_SIZE - active_font.font_size.x + x_add_offset, 0.0);
    for x in 0..BOARD_SIZE {
        let mut text = graphics::Text::new(BOARD_LETTERS[x as usize]);
        text.set_font(active_font.font, active_font.font_size);
        let dist = get_render_pos(x, BOARD_SIZE-1);
        graphics::draw(
            ctx,
            &text,
            graphics::DrawParam::new()
                .dest(dist + y_offset).color(get_color(x % 2 == 1))
        )?;
    }

    for y in 0..BOARD_SIZE {
        let mut text = graphics::Text::new(BOARD_NUMBERS[y as usize]);
        text.set_font(active_font.font, active_font.font_size);
        let dist = get_render_pos(BOARD_SIZE-1,y );
        graphics::draw(
            ctx,
            &text,
            graphics::DrawParam::new()
                .dest(dist + x_offset).color(get_color(y % 2 == 1))
        )?;
    }
    Ok(())
}

pub(crate) fn render_pieces(ctx: &mut Context, config : &RenderConfig, state: &mut ActiveGame) -> GameResult<()> {
    let mut selected_piece : Option<(Vec2, ChessPiece, bool)> = None;

    let active_sprites = &config.spritesets[config.active_sprites_index];

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let board_pos = BoardPosition::new(x, y);

            let safe_piece = match state.game.get_board_piece_clone(board_pos) {
                Some(p) => p,
                None => continue,
            };

            let is_on_white = (x + y) % 2 == 1;

            if state.hover_position.is_some()
                && state.selected_square.is_some()
                && board_pos == state.selected_square.unwrap()
            {
               let selected_render_dist = state.hover_position.unwrap() - Vec2::new(BOARD_RENDER_TILE_SIZE / 2.0,BOARD_RENDER_TILE_SIZE /2.0);
               selected_piece = Some((selected_render_dist, safe_piece, is_on_white));
               continue;
            }

            let dist = get_render_pos(x, y);
            
            graphics::draw(
                ctx,
                get_piece_image(
                    safe_piece.id,
                    safe_piece.color,
                    is_on_white,
                    active_sprites,
                ),
                graphics::DrawParam::new()
                    .dest(dist + active_sprites.offset)
            )?;
        }
    }

    // because there does not exist a way to use z-index,
    // you will have to render in order for this to appear on top
    let (dist, piece,is_on_white) = match selected_piece {
        Some(p) => p,
        None => return Ok(()),
    };

    graphics::draw(
        ctx,
        get_piece_image(
            piece.id,
            piece.color,
            is_on_white,
            active_sprites,
        ),
        graphics::DrawParam::new()
            .dest(dist + active_sprites.offset)
    )?;

    Ok(())
}
