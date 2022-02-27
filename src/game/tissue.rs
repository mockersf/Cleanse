use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    math::const_vec3,
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType,
            BufferInitDescriptor, BufferSize, BufferUsages, ShaderStages,
        },
        renderer::RenderDevice,
    },
    sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle},
};
use rand::Rng;

use crate::{game::z_layers, tear_down, GameState};

use super::{host::HostState, ImmuneSystem};

#[derive(Component)]
pub struct ScreenTag;

pub struct TissuePlugin;
impl Plugin for TissuePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<TissueMaterial>::default())
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(tear_down::<ScreenTag>),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(update_tissue_material),
            );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TissueMaterial>>,
    windows: Res<Windows>,
) {
    debug!("Loading Tissue");

    let window = windows.get_primary().unwrap();
    let resolution = Vec2::new(window.width(), window.height());

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                translation: const_vec3!([0.0, 0.0, z_layers::TISSUE]),
                scale: resolution.extend(1.0),
                ..Default::default()
            },
            material: materials.add(TissueMaterial {
                resolution,
                time: 0.0,
                seed: rand::thread_rng().gen::<i16>() as f32,
                pos: Vec2::ZERO,
                speed: 0.0,
                sickness: 0.0,
                dilatation: 500.0,
            }),
            ..Default::default()
        })
        .insert(ScreenTag);
}

#[allow(clippy::type_complexity)]
fn update_tissue_material(
    camera: Query<
        &Transform,
        (
            With<OrthographicProjection>,
            With<Camera>,
            Without<Handle<TissueMaterial>>,
        ),
    >,
    immune_system: Query<&ImmuneSystem>,
    time: Res<Time>,
    mut tissue_materials: ResMut<Assets<TissueMaterial>>,
    mut tissue: Query<&mut Transform, With<Handle<TissueMaterial>>>,
    host: Res<HostState>,
) {
    for (_, mut tissue_material) in tissue_materials.iter_mut() {
        let camera_transform = camera.single();
        let camera_pos = camera_transform.translation.truncate();
        tissue_material.time += time.delta_seconds();
        tissue_material.pos = camera_pos * Vec2::new(1.0, -1.0);
        tissue_material.speed = immune_system.single().speed;
        tissue_material.sickness = host.sickness;
        let mut field_transform = tissue.single_mut();
        field_transform.translation = camera_pos.extend(z_layers::TISSUE);
    }
}

#[derive(Component, Debug, Clone, TypeUuid, Default, AsStd140)]
#[uuid = "754DDD8C-641C-48F2-A330-596F22A8AB57"]
struct TissueMaterial {
    resolution: Vec2,
    pos: Vec2,
    time: f32,
    seed: f32,
    speed: f32,
    sickness: f32,
    dilatation: f32,
}

#[derive(Clone)]
struct GpuTissueMaterial {
    bind_group: BindGroup,
}

impl RenderAsset for TissueMaterial {
    type ExtractedAsset = TissueMaterial;
    type PreparedAsset = GpuTissueMaterial;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: extracted_asset.as_std140().as_bytes(),
            label: Some("Tissue Settings Buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Tissue BindGroup"),
            layout: &material_pipeline.material2d_layout,
        });

        Ok(GpuTissueMaterial { bind_group })
    }
}

impl Material2d for TissueMaterial {
    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/tissue.wgsl"))
    }
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/tissue.wgsl"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(TissueMaterial::std140_size_static() as u64),
                },
                count: None,
            }],
            label: Some("Tissue BindGroup Layout"),
        })
    }
}
