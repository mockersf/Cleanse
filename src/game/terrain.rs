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

use super::Player;

#[derive(Component)]
struct ScreenTag;

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<TerrainMaterial>::default())
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(tear_down::<ScreenTag>),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(update_terrain_material),
            );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TerrainMaterial>>,
    windows: Res<Windows>,
) {
    debug!("Loading Terrain");

    let window = windows.get_primary().unwrap();
    let resolution = Vec2::new(window.width(), window.height());

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                translation: const_vec3!([0.0, 0.0, z_layers::TERRAIN]),
                scale: resolution.extend(1.0),
                ..Default::default()
            },
            material: materials.add(TerrainMaterial {
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
fn update_terrain_material(
    camera: Query<
        &Transform,
        (
            With<OrthographicProjection>,
            With<Camera>,
            Without<Handle<TerrainMaterial>>,
        ),
    >,
    player: Query<&Player>,
    time: Res<Time>,
    mut terrain_materials: ResMut<Assets<TerrainMaterial>>,
    mut terrain: Query<&mut Transform, With<Handle<TerrainMaterial>>>,
) {
    for (_id, mut terrain_material) in terrain_materials.iter_mut() {
        let camera_transform = camera.single();
        let camera_pos = camera_transform.translation.truncate();
        terrain_material.time += time.delta_seconds();
        terrain_material.pos = camera_pos * Vec2::new(1.0, -1.0);
        terrain_material.speed = player.single().speed;
        let mut field_transform = terrain.single_mut();
        field_transform.translation = camera_pos.extend(z_layers::TERRAIN);
    }
}

#[derive(Component, Debug, Clone, TypeUuid, Default, AsStd140)]
#[uuid = "754DDD8C-641C-48F2-A330-596F22A8AB57"]
struct TerrainMaterial {
    resolution: Vec2,
    pos: Vec2,
    time: f32,
    seed: f32,
    speed: f32,
    sickness: f32,
    dilatation: f32,
}

#[derive(Clone)]
struct GpuTerrainMaterial {
    bind_group: BindGroup,
}

impl RenderAsset for TerrainMaterial {
    type ExtractedAsset = TerrainMaterial;
    type PreparedAsset = GpuTerrainMaterial;
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
            label: Some("Terrain Settings Buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Terrain BindGroup"),
            layout: &material_pipeline.material2d_layout,
        });

        Ok(GpuTerrainMaterial { bind_group })
    }
}

impl Material2d for TerrainMaterial {
    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/terrain.wgsl"))
    }
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/terrain.wgsl"))
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
                    min_binding_size: BufferSize::new(TerrainMaterial::std140_size_static() as u64),
                },
                count: None,
            }],
            label: Some("Terrain BindGroup Layout"),
        })
    }
}
