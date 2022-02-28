use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, text::LayoutJob, Align2, Color32, FontData, FontDefinitions, FontFamily, RichText,
        Stroke, TextFormat, TextStyle, Ui, Widget, WidgetText,
    },
    EguiContext,
};

use crate::{assets::LoadingState, tear_down, GameState, GlobalState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(tear_down::<ScreenTag>))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu));
    }
}

#[derive(Component)]
struct ScreenTag;

fn setup(mut egui_context: ResMut<EguiContext>) {
    debug!("Loading Screen");

    let ctx = egui_context.ctx_mut();
    let mut style: egui::Style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(20.0, 20.0);
    style.spacing.button_padding = egui::vec2(10.0, 10.0);
    style.spacing.window_padding = egui::vec2(20.0, 20.0);
    style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(30, 0, 0);
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::none();
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    style.visuals.widgets.hovered.bg_stroke = Stroke::none();
    style.visuals.widgets.hovered.expansion = 10.0;
    style.visuals.window_corner_radius = 5.0;
    ctx.set_style(style);

    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "Kenney Bold".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/Kenney Bold.ttf")),
    );
    fonts
        .fonts_for_family
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "Kenney Bold".to_owned());

    fonts
        .family_and_size
        .entry(TextStyle::Heading)
        .and_modify(|f| f.1 = 50.0);
    fonts
        .family_and_size
        .entry(TextStyle::Button)
        .and_modify(|f| f.1 = 35.0);
    fonts
        .family_and_size
        .entry(TextStyle::Body)
        .and_modify(|f| f.1 = 35.0);
    fonts
        .family_and_size
        .entry(TextStyle::Small)
        .and_modify(|f| f.1 = 15.0);

    ctx.set_fonts(fonts);
}

fn menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    asset_state: Res<State<LoadingState>>,
    global_state: Res<GlobalState>,
) {
    egui::Window::new(RichText::new("Cleanse").color(Color32::RED))
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .min_width(800.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.separator();
                ui.vertical_centered_justified(|ui| {
                    ui.set_max_width(350.0);
                    let mut new_game = LayoutJob::default();
                    new_game.append(
                        "New Game",
                        0.0,
                        TextFormat::simple(egui::TextStyle::Button, Color32::WHITE),
                    );
                    if global_state.generation > 0 {
                        new_game.append(
                            &format!("\ngeneration {}", global_state.generation),
                            0.0,
                            TextFormat::simple(egui::TextStyle::Small, Color32::GRAY),
                        );
                    }

                    button(
                        ui,
                        new_game,
                        || {
                            let _ = state.set(GameState::Playing);
                        },
                        asset_state.current() != &LoadingState::Assets,
                    );
                    ui.add_space(20.0);
                    button(
                        ui,
                        "Quit",
                        || {
                            let _ = state.set(GameState::Exit);
                        },
                        cfg!(not(target_arch = "wasm32")),
                    );
                    ui.add_space(10.0);
                });
            });
        });
}

pub fn button(
    ui: &mut Ui,
    text: impl Into<WidgetText>,
    mut on_click: impl FnMut(),
    is_enabled: bool,
) {
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
