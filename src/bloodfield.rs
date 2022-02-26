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

use crate::GameState;

#[derive(Component)]
struct ScreenTag;

pub struct BloodfieldPlugin;
impl Plugin for BloodfieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<BloodfieldMaterial>::default())
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(update_bloodfield_material),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Menu).with_system(update_bloodfield_material),
            );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BloodfieldMaterial>>,
    windows: Res<Windows>,
    mut done: Local<bool>,
) {
    if !*done {
        debug!("Loading Bloodfield");

        let window = windows.get_primary().unwrap();
        let resolution = Vec2::new(window.width(), window.height());

        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform {
                    translation: const_vec3!([0.0, 0.0, 0.0]),
                    scale: resolution.extend(1.0),
                    ..Default::default()
                },
                material: materials.add(BloodfieldMaterial::default()),
                ..Default::default()
            })
            .insert(ScreenTag);
        *done = true;
    }
}

#[allow(clippy::type_complexity)]
fn update_bloodfield_material(
    camera: Query<
        &Transform,
        (
            With<OrthographicProjection>,
            With<Camera>,
            Without<Handle<BloodfieldMaterial>>,
        ),
    >,
    time: Res<Time>,
    mut bloodfield_materials: ResMut<Assets<BloodfieldMaterial>>,
    mut bloodfield: Query<&mut Transform, With<Handle<BloodfieldMaterial>>>,
) {
    for (_id, mut bloodfield_material) in bloodfield_materials.iter_mut() {
        let camera_transform = camera.single();
        bloodfield_material.time += time.delta_seconds();
        let mut field_transform = bloodfield.single_mut();
        field_transform.translation = camera_transform.translation.truncate().extend(0.0);
    }
}

#[derive(Component, Debug, Clone, TypeUuid, Default)]
#[uuid = "AC784C13-7197-40AB-BC3A-2ADD64F9E242"]
struct BloodfieldMaterial {
    time: f32,
}

#[derive(Clone)]
struct GpuBloodfieldMaterial {
    bind_group: BindGroup,
}

impl RenderAsset for BloodfieldMaterial {
    type ExtractedAsset = BloodfieldMaterial;
    type PreparedAsset = GpuBloodfieldMaterial;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let value = extracted_asset.time;
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: value.as_std140().as_bytes(),
            label: Some("Bloodfield Settings Buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Bloodfield BindGroup"),
            layout: &material_pipeline.material2d_layout,
        });

        Ok(GpuBloodfieldMaterial { bind_group })
    }
}

impl Material2d for BloodfieldMaterial {
    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/bloodfield.wgsl"))
    }
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/bloodfield.wgsl"))
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
                    min_binding_size: BufferSize::new(f32::std140_size_static() as u64),
                },
                count: None,
            }],
            label: Some("Bloodfield BindGroup Layout"),
        })
    }
}
