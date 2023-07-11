use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, MouseButton},
    graphics::{self, Canvas, Color, DrawMode, DrawParam, Image, Mesh, MeshBuilder, Rect},
    Context, GameResult,
};
use std::path;
mod structs;
use structs::*;
use noise::{NoiseFn, Perlin, Seedable};
const MAP_X: usize = 16;
const MAP_Y: usize = 16;
struct GameState{
    tilemap: [[TileType; MAP_X]; MAP_Y],
    tilemap_mesh: Mesh
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
        let tilemap = generate_tilemap( 1231, 25.0);
        let tilemap_mesh = generate_tilemap_mesh(ctx, tilemap);
        let state = GameState{tilemap, tilemap_mesh};
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
            MouseButton::Right => {
                //Right click
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
}
fn draw_game(canvas: &mut Canvas, state: &mut GameState){
    canvas.draw(&state.tilemap_mesh, DrawParam::default());
}
fn generate_tilemap( seed: u32, magnification: f64) -> [[structs::TileType; MAP_X]; MAP_Y]{
    let perlin = Perlin::new(seed);
    let mut tile_array = [[TileType::Water; MAP_X]; MAP_Y];
    let mut min = 10.0;
    let mut max = -10.0;
    for row in 0..MAP_Y{
        for col in 0..MAP_X{
            let rand_num = perlin.get([(row as f64+0.5)/magnification,(col as f64+0.5)/magnification]);
            if rand_num > max{
                max = rand_num;
            }
            if rand_num < min{
                min = rand_num;
            }
            if rand_num < -0.1{
                tile_array[row][col] = TileType::Land;
            }
            else if rand_num > 0.1{
                tile_array[row][col] = TileType::Mountain;
            }
        }
    }
    println!("min: {}, max: {}", min, max);
    tile_array
}
fn generate_tilemap_mesh(ctx: &Context, tilemap: [[structs::TileType; MAP_X]; MAP_Y]) -> Mesh{
    let mut mesh_builder = MeshBuilder::new();
    for row in 0..MAP_X {
        for col in 0..MAP_Y {
            let tile = tilemap[col][row];

            let color = match tile{
                TileType::Land => Color::from_rgb(50, 252, 0),
                TileType::Water => Color::from_rgb(30, 144, 255),
                TileType::Mountain => Color::from_rgb(192, 192, 192),
            };
            let x = row as f32 * 100.0;
            let y = col as f32 * 100.0;
            let rect = Rect::new(x, y, 100.0, 100.0);
            let _ = mesh_builder.rectangle(DrawMode::fill(), rect, color);
        }
    }
    Mesh::from_data(ctx, mesh_builder.build())
}