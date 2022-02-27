use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, RichText, Stroke, Ui, Widget},
    EguiContext,
};

use crate::{
    game::{self, HostState},
    tear_down, GameState,
};

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_exit(GameState::Dead)
                .with_system(tear_down::<ScreenTag>)
                .with_system(tear_down::<game::ScreenTag>)
                .with_system(tear_down::<game::tissue::ScreenTag>),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Dead)
                .with_system(game::ui::status)
                .with_system(menu),
        );
    }
}

#[derive(Component)]
struct ScreenTag;

fn menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    host_state: Res<HostState>,
) {
    egui::Window::new(RichText::new("Death").color(Color32::RED))
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .min_width(800.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.separator();
                ui.heading(format!("You died at age {:>2.1}", host_state.age));
                ui.add_space(10.0);
                ui.heading("That was inevitable.");
                ui.add_space(30.0);
                ui.vertical_centered_justified(|ui| {
                    ui.set_max_width(350.0);
                    button(
                        ui,
                        "Try Again...",
                        || {
                            let _ = state.set(GameState::Menu);
                        },
                        true,
                    );
                    ui.add_space(10.0);
                });
            });
        });
}

fn button(ui: &mut Ui, text: &str, mut on_click: impl FnMut(), is_enabled: bool) {
    ui.scope(|ui| {
        if !is_enabled {
            ui.set_enabled(false);
        }

        let button = bevy_egui::egui::Button::new(text)
            .stroke(Stroke::new(5.0, Color32::BROWN))
            .fill(Color32::DARK_RED);

        if button.ui(ui).clicked() {
            on_click()
        }
    });
}
