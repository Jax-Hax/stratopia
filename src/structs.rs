use ggez::graphics::Image;

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
#[derive(Clone,Copy,Default)]
pub struct Soldier{
    pub soldier_type: SoldierType,
    pub health: u8,
    pub damage: u8,
    pub defense: u8,
    pub range: u8,
    pub movement: u8
}
#[derive(Clone,Copy,Default)]
pub enum SoldierType{
    #[default]
    None,
    Default,
}