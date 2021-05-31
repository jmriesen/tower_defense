use amethyst::{
    prelude::*,
    core::{
        math::{Vector3,},
        Transform,
    },
    tiles::{TileMap},
    prelude::World,
    renderer::Camera,
};
use super::{
    Ground,
    tiles::GroundTile,
    TILE_SIZE,
};
impl Ground{
    pub fn create_tile_map(&self,world:&mut World){
        let map = TileMap::<GroundTile>::new(
            Vector3::new(self.colum, self.rows, 1), // The dimensions of the map
            Vector3::new(TILE_SIZE,TILE_SIZE, 1), // The dimensions of each tile
            Some(super::super::sprites_management::load_sheet(world, "ground")),
        );

        let (width,hight) = ((TILE_SIZE*self.colum) as f32,(TILE_SIZE*self.rows) as f32);

        let offset = -(TILE_SIZE as f32 );
        let mut transform = Transform::default();
        transform.append_translation_xyz(0.,offset,0.);
        transform.append_translation_xyz(width/2.,hight/2.,0.);
        world.create_entity()
            .with(map)
            .with(transform)
            .build();
    }
    pub fn create_camera(&self,world: &mut World) {
        let (width,hight) = ((TILE_SIZE*self.colum) as f32,(TILE_SIZE*self.rows) as f32);
        let mut transform = Transform::default();
        let offset = -(TILE_SIZE as f32 /2.);
        transform.append_translation_xyz(0.,0.,1.);
        transform.append_translation_xyz(offset,offset,0.);
        transform.append_translation_xyz(width/2.,hight/2.,0.);

        world
            .create_entity()
            .with(Camera::standard_2d(width,hight))
            .with(transform)
            .build();
    }
    pub fn tile_to_trans((x,y):(u32,u32))->Transform{
        let mut transform = Transform::default();
        transform.set_translation_xyz((x * TILE_SIZE) as f32,(y * TILE_SIZE)  as f32,0.);
        transform
    }
    pub fn trans_to_tile(location:&Transform)->(f32,f32){
        let pos = location.translation();
        (pos.x /TILE_SIZE as f32,pos.y /TILE_SIZE as f32)
    }

}
