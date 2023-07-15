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
#[derive(Clone, Copy)]
pub enum IsSoldier{
    None,
    Soldier(Soldier)
}
#[derive(Clone, Copy)]
pub struct Soldier{
    pub soldier_type: SoldierType,
    pub position_x: u8,
    pub position_y: u8,
    pub health: u8
}
#[derive(Clone, Copy)]
pub struct SoldierType{
    pub image: Image,
    pub damage: u8,
    pub health: u8
}