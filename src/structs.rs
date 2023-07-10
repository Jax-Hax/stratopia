pub struct Tile{
    tile_type: TileType,
    resource_type: ResourceType,
}
pub enum ResourceType{
    Village,
    None,

}
pub enum TileType{
    Land,
    Water,
    Mountain,
}