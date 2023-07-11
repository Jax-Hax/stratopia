#[derive(Clone, Copy)]
pub struct Tile{
    pub tile_type: TileType,
    pub resource_type: ResourceType,
}
#[derive(Clone, Copy)]
pub enum ResourceType{
    Village(Village),
    None,
    WaterResource(Resource)
}
#[derive(Clone, Copy)]
pub enum TileType{
    Land,
    Water,
    Mountain,
}
#[derive(Clone, Copy)]
pub struct Village{

}
#[derive(Clone, Copy)]
pub struct Resource{

}