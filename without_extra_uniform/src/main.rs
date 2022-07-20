use bevy::prelude::*;
use bevy::ecs::{system::{lifetimeless::SRes, SystemParamItem}};
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{RenderPipelineDescriptor, SpecializedMeshPipelineError};
use bevy::render::{render_asset::{RenderAsset, PrepareAssetError}, renderer::RenderDevice, render_resource::{BindGroup, BindGroupLayout, BindGroupDescriptor, BindGroupLayoutDescriptor}, camera::ScalingMode};
use bevy::reflect::TypeUuid;
use bevy::sprite::{MaterialMesh2dBundle, Material2d, Material2dPlugin, Material2dPipeline};

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
        material: my_material_assets.add(MyMaterial),
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

#[derive(TypeUuid, Clone)]
#[uuid = "d229cd90-06b0-11ed-b939-0242ac120002"]
struct MyMaterial;

impl Material2d for MyMaterial {
    fn bind_group(material: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[],
        })
    }

    fn vertex_shader(ass: &AssetServer) -> Option<Handle<Shader>> {
        ass.watch_for_changes().unwrap();
        Some(ass.load("shaders/vertex.vert"))
    }

    /// Load the fragment shader
    fn fragment_shader(ass: &AssetServer) -> Option<Handle<Shader>> {
        Some(ass.load("shaders/fragment.frag"))
    }

    /// Needed for changing the entry points of the shader. Bevy uses the WSLS Entry Points, but GLSL has "main"
    fn specialize(descriptor: &mut RenderPipelineDescriptor, _: &MeshVertexBufferLayout) -> Result<(), SpecializedMeshPipelineError> {
            // How to change the vertex shader entry point
            descriptor.vertex.entry_point = "main".into();

            // How to change the fragment shader entry point
            descriptor.fragment.as_mut().unwrap().entry_point = "main".into();

            Ok(())
    }
}

/// GPU Representation of MyMaterial
struct MyMaterialGPU {
    bind_group: BindGroup
}

impl RenderAsset for MyMaterial {
    type ExtractedAsset = MyMaterial;
    type PreparedAsset = MyMaterialGPU;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<MyMaterial>>);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(_: Self::ExtractedAsset, (render_device, pipeline): &mut SystemParamItem<Self::Param>) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[]
        });

        Ok(MyMaterialGPU { bind_group })
    }
}