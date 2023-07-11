use noise::{NoiseFn, Perlin};
use ggez::{
    graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect},
    Context,
};
use crate::structs::*;
//Constants
const MAP_X: usize = 20;
const MAP_Y: usize = 20;
const MAGNIFICATION: f64 = 10.0;
//What numbers generate water and what generates mountains. The rest of the tiles are land tiles.
const WATER_VALUES: [f64; 2] = [0.15,-0.1]; //less than value 1 and greater than value 2
const MOUNTAIN_VALUES: [f64; 2] = [-0.5,-1.0]; //less than value 1 and greater than value 2

pub fn generate_tilemap( seed: u32 ) -> [[TileType; MAP_X]; MAP_Y]{
    //Generates a new perlin seed and gets a value from it for each tile, based on magnification. It then checks to see what tile type it should be.
    let perlin = Perlin::new(seed);
    let mut tile_array = [[TileType::Land; MAP_X]; MAP_Y];
    for row in 0..MAP_Y{
        for col in 0..MAP_X{
            let rand_num = perlin.get([(row as f64+0.5)/MAGNIFICATION,(col as f64+0.5)/MAGNIFICATION]); //Offset by 0.5 otherwise it would always return 0
            if rand_num < WATER_VALUES[0] && rand_num > WATER_VALUES[1]{
                tile_array[row][col] = TileType::Water;
            }
            else if rand_num < MOUNTAIN_VALUES[0] && rand_num > MOUNTAIN_VALUES[1]{
                tile_array[row][col] = TileType::Mountain;
            }
        }
    }
    //TODO: set the starting area to be land, this code doesnt work
    /*for row in 0..3{
        for col in 0..3{
            println!("{},{}",row,col);
            tile_array[row+3][col+3] = TileType::Land; //top left
            tile_array[row+3][MAP_X-3] = TileType::Land; //top right
            tile_array[MAP_Y-3][col+3] = TileType::Land; //bottom left
            tile_array[MAP_Y-3][MAP_X-3] = TileType::Land; //bottom right
        }
    }*/
    tile_array
}
pub fn generate_tilemap_mesh(ctx: &Context, tilemap: [[TileType; MAP_X]; MAP_Y]) -> Mesh{
    //Makes a mesh of the terrain
    let mut mesh_builder = MeshBuilder::new();
    for row in 0..MAP_X {
        for col in 0..MAP_Y {
            let tile = tilemap[col][row];

            let color = match tile{
                TileType::Land => Color::from_rgb(50, 252, 0),
                TileType::Water => Color::from_rgb(30, 144, 255),
                TileType::Mountain => Color::from_rgb(192, 192, 192),
            };
            let x = row as f32 * 50.0;
            let y = col as f32 * 50.0;
            let rect = Rect::new(x, y, 50.0, 50.0);
            let _ = mesh_builder.rectangle(DrawMode::fill(), rect, color);
        }
    }
    Mesh::from_data(ctx, mesh_builder.build())
}