#[derive(Clone, Copy)]
pub enum ResourceType{
    Village(Village),
    None,
    WaterResource(Resource),
    LandResource(Resource),
    MountainResource(Resource),
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