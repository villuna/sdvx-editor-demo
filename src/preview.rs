//! Code for rendering the chart to a preview (in 2d or 3d).
//! The way this is done is by using a seperate camera which renders to a texture, then displaying
//! that texture in an egui image widget.

use bevy::{
    camera::{RenderTarget, visibility::RenderLayers},
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};
use bevy_egui::{EguiGlobalSettings, EguiTextureHandle, EguiUserTextures, PrimaryEguiContext};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_cameras)
        .add_systems(FixedUpdate, spin_cube_system);
}

/// The target image to render the preview to.
/// TODO: I'm just implementing the 2d preview first. Then I will do the 3d later.
#[derive(Deref, Resource)]
pub struct PreviewFrame(pub Handle<Image>);

/// System to set up the cameras used for the preview as well as the main camera
fn setup_cameras(
    mut commands: Commands,
    mut egui_settings: ResMut<EguiGlobalSettings>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
    assets: Res<AssetServer>,
) {
    egui_settings.auto_create_primary_context = false;

    // This will do me for now but in the future I would like to be able to resize this.
    // TODO: that
    let size = Extent3d {
        width: 1200,
        height: 1200,
        ..default()
    };

    // The image we will render to
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // Fill the data with zeroes
    image.resize(size);

    let image_handle = images.add(image);
    egui_user_textures.add_image(EguiTextureHandle::Strong(image_handle.clone()));
    commands.insert_resource(PreviewFrame(image_handle.clone()));

    let yuuko = assets.load("yuukoplead.png");
    let cube = meshes.add(Cuboid::new(4.0, 4.0, 4.0));
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(yuuko),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    // the layer used for the preview pass, which will be attached to the preview pass camera and cube
    let embedded_pass_layer = RenderLayers::layer(1);

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material),
        Transform::from_translation(Vec3::new(0., 0., 1.)),
        MyCube,
        embedded_pass_layer.clone(),
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            // Render before the "main pass" camera
            order: -1,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        RenderTarget::Image(image_handle.into()),
        Transform::from_translation(Vec3::new(0., 0., 15.)).looking_at(Vec3::default(), Vec3::Y),
        embedded_pass_layer.clone(),
    ));

    commands.spawn((Camera2d, PrimaryEguiContext));
}

/// Just a test object to render while I'm setting everything up
#[derive(Component)]
struct MyCube;

fn spin_cube_system(time: Res<Time>, mut cube: Single<&mut Transform, With<MyCube>>) {
    cube.rotate_x(1.5 * time.delta_secs());
    cube.rotate_y(0.7 * time.delta_secs());
    cube.rotate_z(1.3 * time.delta_secs());
}
