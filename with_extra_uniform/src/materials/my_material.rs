use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::std140::{AsStd140, Std140};
use bevy::render::render_resource::{RenderPipelineDescriptor, SpecializedMeshPipelineError, BufferInitDescriptor, BufferUsages, BindGroupEntry, BindGroupLayoutEntry, ShaderStages, BindingType, BufferBindingType, BufferSize};
use bevy::render::{renderer::RenderDevice, render_resource::{BindGroup, BindGroupLayout, BindGroupDescriptor, BindGroupLayoutDescriptor}};
use bevy::render::render_asset::{RenderAsset, PrepareAssetError};
use bevy::reflect::TypeUuid;
use bevy::sprite::{Material2d, Material2dPipeline};
use bevy::ecs::{system::{lifetimeless::SRes, SystemParamItem}};

use bevy::prelude::*;

#[derive(TypeUuid, Clone)]
#[uuid = "d229cd90-06b0-11ed-b939-0242ac120002"]
pub struct MyMaterial {
    pub color: Color
}

impl Material2d for MyMaterial {
    fn bind_group(material: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer { 
                        ty: BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64) 
                    },
                    count: None
                }
            ],
        })
    }

    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/vertex.vert"))
    }

    /// Load the fragment shader
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/fragment.frag"))
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
pub struct MyMaterialGPU {
    bind_group: BindGroup
}

impl RenderAsset for MyMaterial {
    type ExtractedAsset = MyMaterial;
    type PreparedAsset = MyMaterialGPU;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<MyMaterial>>);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(asset: Self::ExtractedAsset, (render_device, pipeline): &mut SystemParamItem<Self::Param>) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let color = Vec4::from_slice(&asset.color.as_linear_rgba_f32());

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: color.as_std140().as_bytes(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                }
            ]
        });

        Ok(MyMaterialGPU { bind_group })
    }
}