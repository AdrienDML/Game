use std::marker::PhantomData;

use bevy::{
    math::Vec3A,
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};

use crate::shape::Line;

// TODO: ways to improve
// make mesh treatment parallel using
// use better intersection algo
// adding distance check

pub struct RayLayerPlugin<Layer>(PhantomData<Layer>)
where
    Layer: Send + Sync + 'static;

impl<Layer> RayLayerPlugin<Layer>
where
    Layer: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Layer> Plugin for RayLayerPlugin<Layer>
where
    Layer: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_event::<FireRay<Layer>>()
            .add_event::<RayHit<Layer>>()
            .add_system(fire_ray::<Layer>);
    }
}

#[derive(Component)]
pub struct RayHitable<Layer: Send + Sync + 'static>(PhantomData<Layer>);

impl<Layer> RayHitable<Layer>
where
    Layer: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

pub struct FireRay<Layer>
where
    Layer: Send + Sync + 'static,
{
    line: Line,
    _m: PhantomData<Layer>,
}

impl<Layer> FireRay<Layer>
where
    Layer: Send + Sync + 'static,
{
    pub fn new(line: Line) -> Self {
        Self {
            line,
            _m: PhantomData,
        }
    }
}

pub struct RayHit<Layer>
where
    Layer: Send + Sync + 'static,
{
    pub entity: Entity,
    pub(crate) _m: PhantomData<Layer>,
}

pub fn fire_ray<Layer: Send + Sync + 'static>(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut rays: EventReader<FireRay<Layer>>,
    mut hits: EventWriter<RayHit<Layer>>,
    cull_query: Query<(
        Entity,
        &Visibility,
        &ComputedVisibility,
        &RayHitable<Layer>,
        &Handle<Mesh>,
        &GlobalTransform,
    )>,
) {
    for ray in rays.iter() {
        let ray_mat = materials.add(StandardMaterial {
            base_color: Color::BLACK,
            ..Default::default()
        });
        let ray_mesh = meshes.add(ray.line.line_mesh(0., 10.));
        commands.spawn().insert_bundle(PbrBundle {
            mesh: ray_mesh,
            material: ray_mat,
            ..Default::default()
        });
        for (entity, vis, c_vis, _, mesh_handle, gtrans) in cull_query.iter() {
            if !vis.is_visible || !c_vis.is_visible {
                continue;
            }
            if let Some(mesh) = meshes.get(mesh_handle) {
                if mesh.primitive_topology() != PrimitiveTopology::TriangleList {
                    error!("Cannot pick non Triangle list meshes!");
                    continue;
                }
                let pos = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
                    Some(vertexs_pos) => match vertexs_pos {
                        VertexAttributeValues::Float32x3(positions) => positions,
                        _ => panic!("Mesh position type unexpected"),
                    },
                    None => panic!("Mesh doesn't have indices!"),
                };
                let world_to_mesh = gtrans.compute_matrix().inverse();
                let ray_line = ray.line.transform(world_to_mesh);
                let hit = if let Some(inds) = mesh.indices() {
                    match inds {
                        Indices::U16(inds) => {
                            let mut hit = false;
                            for index in inds.chunks(3) {
                                let i1 = index[0] as usize;
                                let i2 = index[2] as usize;
                                let i3 = index[3] as usize;
                                if ray_line
                                    .intersect_tri(&[
                                        Vec3A::new(pos[i1][0], pos[i1][1], pos[i1][2]),
                                        Vec3A::new(pos[i2][0], pos[i2][1], pos[i2][2]),
                                        Vec3A::new(pos[i3][0], pos[i3][1], pos[i3][2]),
                                    ])
                                    .is_some()
                                {
                                    hit = true;
                                    break;
                                }
                            }
                            hit
                        }
                        Indices::U32(inds) => {
                            let mut hit = false;
                            for index in inds.chunks(3) {
                                let i1 = index[0] as usize;
                                let i2 = index[2] as usize;
                                let i3 = index[3] as usize;
                                if ray_line
                                    .intersect_tri(&[
                                        Vec3A::new(pos[i1][0], pos[i1][1], pos[i1][2]),
                                        Vec3A::new(pos[i2][0], pos[i2][1], pos[i2][2]),
                                        Vec3A::new(pos[i3][0], pos[i3][1], pos[i3][2]),
                                    ])
                                    .is_some()
                                {
                                    hit = true;
                                    break;
                                }
                            }
                            hit
                        }
                    }
                } else {
                    let mut hit = false;
                    for positions in pos.chunks(3) {
                        let pos1 = positions[0];
                        let pos2 = positions[1];
                        let pos3 = positions[2];
                        if ray_line
                            .intersect_tri(&[
                                Vec3A::new(pos1[0], pos1[1], pos1[2]),
                                Vec3A::new(pos2[0], pos2[1], pos2[2]),
                                Vec3A::new(pos3[0], pos3[1], pos3[2]),
                            ])
                            .is_some()
                        {
                            hit = true;
                            break;
                        }
                    }
                    hit
                };
                if hit {
                    hits.send(RayHit {
                        entity,
                        _m: PhantomData,
                    })
                }
            }
        }
    }
}

pub trait IntoUsize: Copy {
    fn into_usize(self) -> usize;
}
impl IntoUsize for u16 {
    fn into_usize(self) -> usize {
        self as usize
    }
}
impl IntoUsize for u32 {
    fn into_usize(self) -> usize {
        self as usize
    }
}
