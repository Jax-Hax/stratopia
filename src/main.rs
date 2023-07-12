use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, MouseButton},
    graphics::{self, Canvas, DrawMode, DrawParam, Image, Mesh, MeshBuilder, Rect},
    Context, GameResult,
};
use std::path;
mod structs;
use structs::*;
mod generation;
const MAP_X: usize = 20;
const MAP_Y: usize = 20;
struct GameState{
    tilemap: [[TileType; MAP_X]; MAP_Y],
    tilemap_mesh: Mesh,
    // Store the camera position
    camera_position: ggez::mint::Point2<f32>,
    frames: usize,
}
fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");
    let window_setup = WindowSetup::default().title("Stratopia");
    let window_mode = WindowMode::default().fullscreen_type(ggez::conf::FullscreenType::Desktop);
    let cb = ggez::ContextBuilder::new("Chess", "Jax Bulbrook")
        .window_setup(window_setup)
        .window_mode(window_mode);
    let cb = cb.add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)

}
impl GameState{
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut tilemap = generation::generate_tilemap(2);
        let tilemap_mesh = generation::generate_tilemap_mesh(ctx, tilemap);
        //let resource_map = generation::generate_resource_map(&mut tilemap,2);
        let state = GameState{tilemap, tilemap_mesh,camera_position: ggez::mint::Point2 { x: 0.0, y: 0.0 },frames: 0};
        Ok(state)
    }
}
impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        draw_game(&mut canvas, self);
        canvas.finish(ctx)?;
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ctx.time.fps());
        }
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        btn: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        match btn {
            MouseButton::Left => {
                //Left click
            }
            _ => (),
        }
        Ok(())
    }
    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        btn: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        match btn {
            MouseButton::Left => {
                //Left up
            }

            _ => (),
        }
        Ok(())
    }
    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        _x: f32,
        _y: f32,
        dx: f32,
        dy: f32,
    ) -> GameResult {
        if ctx.mouse.button_pressed(MouseButton::Right) {
            self.camera_position.x += dx;
            self.camera_position.y += dy;
        }
        Ok(())
    }
    
}
fn draw_game(canvas: &mut Canvas, state: &mut GameState){
    canvas.draw(&state.tilemap_mesh, DrawParam::dest(DrawParam::default(),state.camera_position));
}
