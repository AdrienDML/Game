use std::{collections::HashMap, f32::consts::PI};

use bevy::prelude::*;
use bevy_ext::{raycast::RayHitable, shape::Cylinder};

use crate::GridRayLayer;

pub fn get_grid_pos(display_radius: u32, tile_size: f32) -> Vec<(Vec3, IVec2)> {
    let mut positions = Vec::new();
    let tile_size = tile_size + 0.1;
    let angle = PI / 3.;
    positions.push((Vec3::ZERO, IVec2::ZERO));
    let grid_dirs = [
        IVec2::X,
        IVec2::Y,
        IVec2::Y - IVec2::X,
        -IVec2::X,
        -IVec2::Y,
        IVec2::X - IVec2::Y,
    ];
    for dir in 0..6 {
        let grid_dir = grid_dirs[dir];
        let o_grid_dir = grid_dirs[(dir + 2) % 6];
        let a = angle + PI / 6. + dir as f32 * angle;
        for i in 1..display_radius {
            let dir_x = a.cos() * tile_size * i as f32;
            let dir_z = a.sin() * tile_size * i as f32;
            for j in 1..(i + 1) {
                let shift_x = (a + 2. * angle).cos() * tile_size * (j - 1) as f32;
                let shift_z = (a + 2. * angle).sin() * tile_size * (j - 1) as f32;
                let x = dir_x + shift_x;
                let z = dir_z + shift_z;
                positions.push((
                    Vec3::new(x, 0f32, z),
                    i as i32 * grid_dir + (i + j) as i32 * o_grid_dir,
                ));
            }
        }
    }
    positions
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Grid {
    pub tiles: HashMap<IVec2, Entity>,
}

struct GridSettings {
    pub radius: usize,
    pub tile_size: f32,
    pub kind: GridKind,
}

enum GridKind {
    Hex,
    Square,
    Iso,
}

impl GridSettings {
    pub fn get_global_pos(&self, coords: IVec2) -> Vec3 {
        match self.kind {
            GridKind::Hex => self.get_global_pos_hex(coords),
            GridKind::Square => {
                unimplemented!()
            }
            GridKind::Iso => {
                unimplemented!();
            }
        }
    }

    fn get_global_pos_hex(&self, coords: IVec2) -> Vec3 {
        let x_global = Vec3::new(2f32 * self.tile_size * 30f32.cos(), 0., 0.);
        let y_global = Quat::from_euler(EulerRot::XYZ, 0., 0., 30.).mul_vec3(x_global);
        return coords.x as f32 * x_global + coords.y as f32 * y_global;
    }
}

// pub fn maintaian(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut grid_setting: ResMut<Assets<GridSettings>>,
//     mut grids: Query<&mut Grid>,
//     mut tiles: Query<(Entity, Tile, Transform, Visibility)>,
// ) {
//     for grid in grids.iter() {
//         grid.tiles
//     }
// }

pub fn create_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(Cylinder {
        height: 0.25,
        radius: 0.5,
        segments: 6,
    }));
    let material = materials.add(StandardMaterial::default());
    let positions = get_grid_pos(6, 1f32 * (PI / 6f32).cos());
    let mut childs = Vec::new();
    let mut tiles = HashMap::new();
    for (pos, gpos) in positions {
        let tile = commands
            .spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            })
            .insert(Tile)
            .insert(RayHitable::<GridRayLayer>::new())
            .id();
        tiles.insert(gpos, tile);
        childs.push(tile);
    }
    let grid_comp = Grid { tiles };
    let grid = commands
        .spawn()
        .insert(grid_comp)
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .id();
    commands.entity(grid).push_children(&childs);
}
