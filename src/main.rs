use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, MouseButton},
    graphics::{self, Canvas, DrawMode, DrawParam, Image, Mesh, MeshBuilder, Rect},
    Context, GameResult,
};
use rand::Rng;
use std::path;
mod structs;
use structs::*;
mod generation;
const MAP_X: usize = 22;
const MAP_Y: usize = 22;
struct GameState{
    tilemap: [[TileType; MAP_X]; MAP_Y],
    tilemap_mesh: Mesh,
    resource_map: [[ResourceType; MAP_X]; MAP_Y],
    resource_map_mesh: Mesh,
    soldier_map: [[Soldier; MAP_X]; MAP_Y],
    // Store the camera position
    camera_position: ggez::mint::Point2<f32>,
    zoom: ggez::mint::Point2<f32>,
    frames: usize,
    soldier_images: SoldierImages,
}
struct SoldierImages{
    default: Image
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
        let seed = rand::thread_rng().gen();
        let mut tilemap = generation::generate_tilemap(seed);
        let resource_map = generation::generate_resource_map(&mut tilemap, seed as u64);
        let tilemap_mesh = generation::generate_tilemap_mesh(ctx, tilemap);
        let resource_map_mesh = generation::generate_resource_map_mesh(ctx, resource_map);
        let soldier_images = initialize_soldier_images(ctx);
        let soldier_map = generation::generate_soldier_map();
        let state = GameState{tilemap, tilemap_mesh,resource_map,resource_map_mesh,soldier_map, soldier_images ,camera_position: ggez::mint::Point2 { x: 0.0, y: 0.0 },zoom: ggez::mint::Point2 { x: 1.0, y: 1.0 }, frames: 0};
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
        //print frames
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
        //move the camera around with right mouse button
        if ctx.mouse.button_pressed(MouseButton::Right) {
            self.camera_position.x += dx;
            self.camera_position.y += dy;
        }
        Ok(())
    }
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        //zoom the camera in and out
        if y > 0.0 {
            self.zoom.x = self.zoom.x + 0.1;
            self.zoom.y = self.zoom.y + 0.1;
        } else if y < 0.0 {
            self.zoom.x = self.zoom.x - 0.1;
            self.zoom.y = self.zoom.y - 0.1;
        }
        self.zoom.x = self.zoom.x.clamp(0.5, 3.0);
        self.zoom.y = self.zoom.y.clamp(0.5, 3.0);
        Ok(())
    }
    
}
fn draw_game(canvas: &mut Canvas, state: &mut GameState){
    //TODO: implement the zoom better, calculating an offset to make it zoom based on the center maybe
    let default_draw_param = DrawParam::dest(DrawParam::default(),state.camera_position).scale(state.zoom);
    canvas.draw(&state.tilemap_mesh, default_draw_param);
    canvas.draw(&state.resource_map_mesh, default_draw_param);
    draw_soldiers(canvas, state.soldier_map,state.soldier_images);
}
fn initialize_soldier_images(ctx: &mut Context) -> SoldierImages{
    let soldier_images = SoldierImages{default: Image::from_path(ctx, "/soldier.png").unwrap()};
    soldier_images
}
fn draw_soldiers(canvas: &mut Canvas, soldier_map:[[Soldier; MAP_X]; MAP_Y],soldier_images: SoldierImages){
    for row in 0..MAP_X {
        for col in 0..MAP_Y {
            let soldier = soldier_map[col][row];
            if let SoldierType::None = soldier.soldier_type{
                continue;
            } 
            let image = match soldier.soldier_type{
                SoldierType::Default => soldier_images.default,
                _ => soldier_images.default
            };
            let x = row as f32 * 50.0;
            let y = col as f32 * 50.0;
            let rect = Rect::new(x, y, 50.0, 50.0);
            canvas.draw(soldier_images.default, draw_param);
        }
    }
}