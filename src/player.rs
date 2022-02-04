use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub fn get_player_mesh(height: f32, prop: f32) -> Mesh {
    let body_width = (1. - prop) * height;
    let head_height = (1. - prop) * height;
    let head_width = head_height / 2.;
    let body_height = prop * height;

    let positions = vec![
        [0., 0., 0.],
        [body_width, body_height, body_width],
        [-body_width, body_height, body_width],
        [-body_width, body_height, -body_width],
        [body_width, body_height, -body_width],
        [head_width, body_height, head_width],
        [-head_width, body_height, head_width],
        [-head_width, body_height, -head_width],
        [head_width, body_height, -head_width],
        [head_width, height, head_width],
        [-head_width, height, head_width],
        [-head_width, height, -head_width],
        [head_width, height, -head_width],
    ];
    let indices = vec![
        2, 0, 1, 3, 0, 2, 4, 0, 3, 1, 0, 4, 2, 1, 6, 6, 1, 5, 3, 2, 7, 7, 2, 6, 4, 3, 8, 8, 3, 7,
        1, 4, 5, 5, 4, 8, 6, 5, 10, 10, 5, 9, 7, 6, 11, 11, 6, 10, 8, 7, 12, 12, 7, 11, 5, 8, 9, 9,
        8, 12, 10, 9, 11, 11, 9, 12,
    ];
    let mut uvs = Vec::new();
    for i in 0..positions.len() {
        uvs.push([0.0, 0.0]);
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();
    mesh
}

#[derive(Component)]
pub struct Player {
    pub pos: IVec2,
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_mesh = meshes.add(get_player_mesh(1., 0.8));
    let player_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1., 0., 0.),
        ..Default::default()
    });
    println!("Player spawned in!");
    commands
        .spawn()
        .insert(Player { pos: IVec2::ZERO })
        .insert_bundle(PbrBundle {
            mesh: player_mesh,
            material: player_material,
            ..Default::default()
        });
}
