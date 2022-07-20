use bevy::{prelude::*, sprite::{Material2dPlugin, MaterialMesh2dBundle}, render::camera::ScalingMode};
use materials::my_material::MyMaterial;

mod materials;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<MyMaterial>::default())
        .add_startup_system(setup)
        .run();
}

/// Setups a simple 3D Scene with a 2D Camera
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut my_material_assets: ResMut<Assets<MyMaterial>>) {
    commands.spawn().insert_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        material: my_material_assets.add(MyMaterial {
            color: Color::rgba(1., 1., 1., 1.)
        }),
        ..default()
    });

    // Creates camera and set it up
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.right = 1.0 * 16. / 9.;
    camera.orthographic_projection.left = -1.0 * 16. / 9.;

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}