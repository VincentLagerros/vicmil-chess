//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::event;
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, GameResult};
use glam::*;

const SCREEN_SIZE: (f32, f32) = (500f32, 700f32);

const BOARD_SIZE: usize = 8;
const BOARD_RENDER_SIZE: f32 = 500f32;
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

/** Just renders the background board */
fn render_board(ctx: &mut Context) -> GameResult<()> {
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

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 4.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BACKGROUND_COLOR);

        render_board(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb =
        ggez::ContextBuilder::new("vinlag_vicil_chess", "Vincent Lagerros and Victor Millberg")
            .window_setup(ggez::conf::WindowSetup::default().title("Chess!"))
            .window_mode(
                ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1),
            );
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
