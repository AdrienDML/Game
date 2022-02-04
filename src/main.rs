use bevy::prelude::*;
use bevy_ext::camera::{screen_to_world_dir, PanOrbitCameraPlugin};
use bevy_ext::raycast::{FireRay, RayHit, RayLayerPlugin};
// use bevy_ext::debug::GridPlugin;

use player::spawn_player;
use tilemap::create_grid;
mod player;
mod tilemap;

pub trait GridRayLayerT {}
pub struct GridRayLayer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RayLayerPlugin::<GridRayLayer>::new())
        .add_startup_system(setup)
        .add_startup_system(create_grid)
        .add_startup_system(spawn_player)
        .add_system(ray_fired)
        .add_system(click_to_fire_ray_on_layer)
        .add_system(on_ray_hit)
        // .add_plugin(GridPlugin(None))
        .add_plugin(PanOrbitCameraPlugin(Some("MainCam".into())))
        // .add_plugin(EditorPlugin)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    })
}

pub fn ray_fired(mut events: EventReader<FireRay<GridRayLayer>>) {
    for event in events.iter() {
        println!("ray fired");
    }
}

pub fn click_to_fire_ray_on_layer(
    mut windows: ResMut<Windows>,
    q_camera: Query<(&GlobalTransform, &Camera)>,
    mut ray_events: EventWriter<FireRay<GridRayLayer>>,
    btn: Res<Input<MouseButton>>,
) {
    let window = windows.get_primary().unwrap();
    if btn.just_pressed(MouseButton::Left) {
        for (gtrans, cam) in q_camera.iter() {
            let ray = screen_to_world_dir(window, gtrans, cam);
            ray_events.send(FireRay::<GridRayLayer>::new(ray));
        }
    }
}

pub fn on_ray_hit(mut hits: EventReader<RayHit<GridRayLayer>>) {
    for hit in hits.iter() {
        let entity = hit.entity;
        eprintln!("We hit something {entity:?}!");
    }
}
