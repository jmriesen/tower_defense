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
    tiles::TileRenderer,
    TILE_SIZE,
};
impl Ground{
    pub fn create_tile_map(&self,world:&mut World){
        let map = TileMap::<TileRenderer>::new(
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
    pub fn create_enemy_factories(&self,world: &mut World) {
        use crate::enemy::EnemyFactory;
        for source in &self.source_points{
            world.create_entity()
                .with(Transform::from(*source))
                .with(EnemyFactory)
                .build();
        }

    }
}
