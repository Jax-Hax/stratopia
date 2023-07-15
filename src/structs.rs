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
pub enum IsSoldier{
    None,
    Soldier(Soldier)
}
pub struct Soldier{
    pub soldier_type: SoldierType,
    pub position: u8,
    pub health: u8
}
pub struct SoldierType{
    pub image: Image,
    pub damage: u8,
    pub health: u8
}