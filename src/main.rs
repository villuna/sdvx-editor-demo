use bevy::prelude::*;

mod preview;

use bevy_egui::{
    EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{self, LayerId, Ui, UiBuilder},
};
use preview::PreviewFrame;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(EguiPrimaryContextPass, egui)
        .add_plugins(preview::plugin)
        .run()
}

fn egui(
    mut ctx: EguiContexts,
    frame: Res<PreviewFrame>,
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
        let available = ui.available_size();
        dbg!((available, ui.cursor()));
        let bounds = f32::min(available.x, available.y);
        ui.image(egui::load::SizedTexture::new(
            frame_id,
            egui::Vec2::splat(bounds),
        ));
    });

    Ok(())
}
