use bevy::prelude::*;
use noise::Fbm;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{NoiseFn, Perlin};

mod voxels;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn noise_setup(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {

    let fbm = Fbm::new();
    PlaneMapBuilder::new(&fbm)
        .set_size(100, 100)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm.png");
}

fn cube_setup(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.22 })),
        material: materials.add(Color::rgb(1.0, 4.0, 5.0).into()),
        ..Default::default()
    });
}

