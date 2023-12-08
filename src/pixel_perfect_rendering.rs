use bevy::app::App;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::{Asset, Assets, Camera, Camera2d, Camera2dBundle, Color, Commands, Component, default, Handle, Image, Mesh, OrthographicProjection, Plugin, Res, ResMut, Resource, shape, StandardMaterial, Startup, TypePath, Vec2};
use bevy::render::camera::{CameraRenderGraph, RenderTarget, ScalingMode};
use bevy::render::render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::view::{Layer, RenderLayers, VisibleEntities};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};
use log::info;

// TODO: Letterboxing
// TODO: multiple pixel perfect views (turn this shit into a component bundle. instead of having a static set of 2 cameras, have a PixelPerfectCamera2dBundle component that sets up the offscreen render target, renders to it. Then we have another component for the output camera (PixelPerfectCamera2dBundle contains the render target, and another camera can use it as the render input)).

pub struct PixelPerfectPlugin {
    size: (u32, u32),
    render_layer: Layer,
    read_layers: RenderLayers,
}

impl PixelPerfectPlugin {
    pub fn new(size: (u32, u32), render_layer: u8, read_layers: RenderLayers) -> PixelPerfectPlugin {
        Self { size, render_layer, read_layers }
    }
}


#[derive(Component)]
pub struct PixelPerfectWorldCamera;

#[derive(Component)]
pub struct PixelPerfectFinalCamera;


#[derive(Resource)]
struct PixelPerfectSettings {
    size: (u32, u32),
    render_layer: Layer,
    read_layers: RenderLayers,
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
struct PixelPerfectRenderMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,
}

impl Material2d for PixelPerfectRenderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/pixel_perfect.wgsl".into()
    }
}

#[derive(Component)]
struct PixelPerfectRenderQuad;

fn pixel_perfect_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PixelPerfectRenderMaterial>>,
    mut images: ResMut<Assets<Image>>,
    settings: Res<PixelPerfectSettings>,
) {
    info!("Setting up PixelPerfectPlugin with size: {:?}x{:?}", settings.size.0, settings.size.1);
    info!("PixelPerfectPlugin will render to layer {:?} and read from layers {:?}", settings.render_layer, settings.read_layers);
    let size = Extent3d {
        width: settings.size.0,
        height: settings.size.1,
        depth_or_array_layers: 1,
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("Pixel Perfect Offscreen Texture"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);

    let image_handle = images.add(image);

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2 { x: size.width as f32, y: size.height as f32})));

    let material_handle = materials.add( PixelPerfectRenderMaterial {
        texture: image_handle.clone(),
    });

    let render_layer = RenderLayers::layer(settings.render_layer);

    let render_quad_entity = commands.spawn((render_layer, MaterialMesh2dBundle {
        mesh: quad_handle.into(),
        material: material_handle,
        ..default()
    })).id();

    // This camera will draw the world to the texture
    commands.spawn((Camera2dBundle {
        camera: Camera {
            order: -1,
            target: RenderTarget::Image(image_handle.clone()),
            ..default()
        },
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: size.width as f32,
                height: size.height as f32,
            },
            near: -1000.0,
            far: 1000.0,
            ..default()
        },
        ..default()
    }, settings.read_layers.clone(), PixelPerfectWorldCamera));

    // This camera will draw the offscreen to the window
    commands.spawn((Camera2dBundle {
        // Limit the render pass to only the quad. This way, I can use the same layer for all offscreen render targets and not see them if I don't want to.
        visible_entities: VisibleEntities { entities: vec![render_quad_entity] },
        // I probably could make it better by just not doing this but here we are
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: size.width as f32,
                height: size.height as f32,
            },
            near: -1000.0,
            far: 1000.0,
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::GREEN)
        },
        ..default()
    }, render_layer.clone(), PixelPerfectFinalCamera));
}

impl Plugin for PixelPerfectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PixelPerfectSettings {
            size: self.size,
            render_layer: self.render_layer,
            read_layers: self.read_layers,
        });

        app.add_plugins(Material2dPlugin::<PixelPerfectRenderMaterial>::default());

        app.add_systems(Startup, pixel_perfect_setup);
    }
}