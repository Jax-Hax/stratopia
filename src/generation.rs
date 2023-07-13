use noise::{NoiseFn, Perlin};
use ggez::{
    graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect},
    Context,
};
use crate::structs::*;
use rand::{Rng, SeedableRng};
//Constants
const MAP_X: usize = 22; //This number unfortunately has to be fixed as I couldn't wrap my head around calculating everything for any possible number
const MAP_Y: usize = 22; //This number unfortunately has to be fixed as I couldn't wrap my head around calculating everything for any possible number
const MAGNIFICATION: f64 = 10.0;
//What numbers generate water and what generates mountains. The rest of the tiles are land tiles.
//RIVER BASED
const WATER_VALUES: [f64; 2] = [0.15,-0.1]; //less than value 1 and greater than value 2
const MOUNTAIN_VALUES: [f64; 2] = [-0.5,-1.0]; //less than value 1 and greater than value 2
//LAKE BASED
//Work in progress
//The different splits of the tilemap, 4 squares on each side with a gap in the middle and a middle village
const STARTING_OFFSET: usize = 2; //how far off you are from the corner of the map
const VILLAGE_DISTANCE: usize = 5; //how far the 3 other starter villages are from the player
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

    //Make a guaranteed path to the villages
    //you start at two spaces away from your corner, so top left would be [2,2]
    //then there is a village on you, a village 4 spaces to both sides, and a village diagonally towards the middle 4 spaces away, making a square of villages that is 6x6
    //offsets some by -1 because of zero indexed arrays
    for offset in 0..VILLAGE_DISTANCE{
        //verticals
        tile_array[STARTING_OFFSET+offset][STARTING_OFFSET] = TileType::Land; //top left
        tile_array[MAP_Y-STARTING_OFFSET-1-offset][STARTING_OFFSET] = TileType::Land; //bottom left
        tile_array[STARTING_OFFSET+offset][MAP_X-STARTING_OFFSET-1] = TileType::Land; //top right
        tile_array[MAP_Y-STARTING_OFFSET-1-offset][MAP_X-STARTING_OFFSET-1] = TileType::Land; //bottom right
        
        //horizontals
        tile_array[STARTING_OFFSET][STARTING_OFFSET+offset] = TileType::Land; //top left
        tile_array[MAP_Y-STARTING_OFFSET-1][STARTING_OFFSET+offset] = TileType::Land; //bottom left
        tile_array[STARTING_OFFSET][MAP_X-STARTING_OFFSET-1-offset] = TileType::Land; //top right
        tile_array[MAP_Y-STARTING_OFFSET-1][MAP_X-STARTING_OFFSET-1-offset] = TileType::Land; //bottom right
        
        //diagonals
        tile_array[STARTING_OFFSET+offset][STARTING_OFFSET+offset] = TileType::Land; //top left
        tile_array[MAP_Y-STARTING_OFFSET-1-offset][STARTING_OFFSET+offset] = TileType::Land; //bottom left
        tile_array[STARTING_OFFSET+offset][MAP_X-STARTING_OFFSET-1-offset] = TileType::Land; //top right
        tile_array[MAP_Y-STARTING_OFFSET-1-offset][MAP_X-STARTING_OFFSET-1-offset] = TileType::Land; //bottom right
    }
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
pub fn generate_resource_map(tilemap: &mut [[TileType; MAP_X]; MAP_Y], seed: u64) -> [[ResourceType; MAP_X]; MAP_Y]{
    let mut resource_array = [[ResourceType::None; MAP_X]; MAP_Y];
    const DEFAULT_VILLAGE: ResourceType = ResourceType::Village(Village {  });
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    //Split into 4 sectors, for each corner of the map. Then, do the dividers in the lines down the middle. Then put a village in the center.
    //TODO: Have each village offset by a value of 1, so it could generate 1 to the left, right, etc for some randomness
    //top left
    resource_array[STARTING_OFFSET][STARTING_OFFSET] = DEFAULT_VILLAGE;
    resource_array[STARTING_OFFSET][VILLAGE_DISTANCE+STARTING_OFFSET] = DEFAULT_VILLAGE;
    resource_array[VILLAGE_DISTANCE+STARTING_OFFSET][STARTING_OFFSET] = DEFAULT_VILLAGE;
    resource_array[VILLAGE_DISTANCE+STARTING_OFFSET][VILLAGE_DISTANCE+STARTING_OFFSET] = DEFAULT_VILLAGE;

    //bottom left
    resource_array[MAP_Y-STARTING_OFFSET-1][STARTING_OFFSET] = DEFAULT_VILLAGE;
    resource_array[MAP_Y-STARTING_OFFSET-1][VILLAGE_DISTANCE+STARTING_OFFSET] = DEFAULT_VILLAGE;
    resource_array[MAP_Y-(VILLAGE_DISTANCE+STARTING_OFFSET)-1][STARTING_OFFSET] = DEFAULT_VILLAGE;
    resource_array[MAP_Y-(VILLAGE_DISTANCE+STARTING_OFFSET)-1][VILLAGE_DISTANCE+STARTING_OFFSET] = DEFAULT_VILLAGE;

    //top right
    resource_array[STARTING_OFFSET][MAP_X-STARTING_OFFSET-1] = DEFAULT_VILLAGE;
    resource_array[STARTING_OFFSET][MAP_X-(VILLAGE_DISTANCE+STARTING_OFFSET)-1] = DEFAULT_VILLAGE;
    resource_array[VILLAGE_DISTANCE+STARTING_OFFSET][MAP_X-STARTING_OFFSET-1] = DEFAULT_VILLAGE;
    resource_array[VILLAGE_DISTANCE+STARTING_OFFSET][MAP_X-(VILLAGE_DISTANCE+STARTING_OFFSET)-1] = DEFAULT_VILLAGE;

    //bottom right
    resource_array[MAP_Y-STARTING_OFFSET-1][MAP_X-STARTING_OFFSET-1] = DEFAULT_VILLAGE;
    resource_array[MAP_Y-STARTING_OFFSET-1][MAP_X-(VILLAGE_DISTANCE+STARTING_OFFSET)-1] = DEFAULT_VILLAGE;
    resource_array[MAP_Y-(VILLAGE_DISTANCE+STARTING_OFFSET)-1][MAP_X-STARTING_OFFSET-1] = DEFAULT_VILLAGE;
    resource_array[MAP_Y-(VILLAGE_DISTANCE+STARTING_OFFSET)-1][MAP_X-(VILLAGE_DISTANCE+STARTING_OFFSET)-1] = DEFAULT_VILLAGE;

    //middle village
    resource_array[MAP_Y/2-rng.gen_range(0..=1)][MAP_X/2-rng.gen_range(0..=1)] = DEFAULT_VILLAGE;
    
    //random villages for each side
    resource_array[MAP_Y/2-rng.gen_range(0..=1)][MAP_X/2+4+rng.gen_range(0..=5)] = DEFAULT_VILLAGE; //right
    resource_array[MAP_Y/2-rng.gen_range(0..=1)][MAP_X/2-5-rng.gen_range(0..=5)] = DEFAULT_VILLAGE; //left
    resource_array[MAP_Y/2+4+rng.gen_range(0..=5)][MAP_X/2-rng.gen_range(0..=1)] = DEFAULT_VILLAGE; //bottom
    resource_array[MAP_Y/2-5-rng.gen_range(0..=5)][MAP_X/2-rng.gen_range(0..=1)] = DEFAULT_VILLAGE; //top
    resource_array
}
fn get_random_resource(tile_type: TileType,resources_left: u32){

}
pub fn generate_resource_map_mesh(ctx: &Context, resource_map: [[ResourceType; MAP_X]; MAP_Y]) -> Mesh{
    //Makes a mesh of the terrain
    let mut mesh_builder = MeshBuilder::new();
    for row in 0..MAP_X {
        for col in 0..MAP_Y {
            let resource = resource_map[col][row];
            if let ResourceType::None = resource{
                continue;
            } 
            let color = match resource{
                ResourceType::Village(_) => Color::from_rgb(101, 67, 33),
                _ => Color::from_rgb(55, 55, 55)
            };
            let x = row as f32 * 50.0;
            let y = col as f32 * 50.0;
            let rect = Rect::new(x, y, 50.0, 50.0);
            let _ = mesh_builder.rectangle(DrawMode::fill(), rect, color);
        }
    }
    Mesh::from_data(ctx, mesh_builder.build())
}