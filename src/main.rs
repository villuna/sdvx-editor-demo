use bevy::{
    camera::{RenderTarget, visibility::RenderLayers},
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};
use bevy_egui::{
    EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass, EguiTextureHandle,
    EguiUserTextures, PrimaryEguiContext,
    egui::{self, LayerId, Ui, UiBuilder},
};

#[derive(Component)]
struct MyCube;

#[derive(Deref, Resource)]
struct EmbeddedFrame(Handle<Image>);

fn startup(
    mut commands: Commands,
    mut egui_settings: ResMut<EguiGlobalSettings>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
    assets: Res<AssetServer>,
) {
    egui_settings.auto_create_primary_context = false;

    let size = Extent3d {
        width: 512,
        height: 512,
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
    commands.insert_resource(EmbeddedFrame(image_handle.clone()));

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

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, spin_cube_system)
        .add_systems(EguiPrimaryContextPass, egui)
        .run()
}

fn egui(
    mut ctx: EguiContexts,
    frame: Res<EmbeddedFrame>,
    mut exit: MessageWriter<AppExit>,
) -> Result {
    let frame_id = ctx.image_id(&**frame).unwrap();
    let ctx = ctx.ctx_mut()?;

    let mut ui = Ui::new(
        ctx.clone(),
        "viewport".into(),
        UiBuilder::new()
            .layer_id(LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );

    egui::Panel::top("menu bar").show(&mut ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    exit.write(AppExit::Success);
                }
            })
        });
    });

    egui::Panel::top("top panel").show(&mut ui, |ui| ui.label("hello there"));

    egui::Panel::left("left panel").show(&mut ui, |ui| {
        ui.label("Left panel");

        if ui.button("Umazing!").clicked() {
            println!("Our brains are umazing");
        }
    });

    egui::CentralPanel::default_margins().show(&mut ui, |ui| {
        ui.image(egui::load::SizedTexture::new(
            frame_id,
            egui::vec2(512., 512.),
        ));
    });

    Ok(())
}

fn spin_cube_system(time: Res<Time>, mut cube: Single<&mut Transform, With<MyCube>>) {
    cube.rotate_x(1.5 * time.delta_secs());
    cube.rotate_y(0.7 * time.delta_secs());
    cube.rotate_z(1.3 * time.delta_secs());
}
