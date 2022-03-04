use std::fmt;

use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, text::LayoutJob, Align2, Color32, ImageButton, Layout, RichText, TextFormat, Ui,
        Widget, WidgetText,
    },
    EguiContext,
};
use rand::prelude::IteratorRandom;
use strum::{EnumIter, IntoEnumIterator};

use crate::{assets::AudioAssets, GameState};

use super::{immune_system::ImmuneSystem, HostState};

pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_update(GameState::LevelUp)
                .with_system(levelup)
                .with_system(super::ui::status),
        );
    }
}

fn levelup(
    mut egui_context: ResMut<EguiContext>,
    mut immune_system: Query<&mut ImmuneSystem>,
    mut host: ResMut<HostState>,
    mut state: ResMut<State<GameState>>,
    mut levelups: Local<Option<Vec<LevelUp>>>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    if let Some(selected) = levelups.as_ref().cloned() {
        egui::Window::new(RichText::new("Level Up!").color(Color32::RED))
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .collapsible(false)
            .resizable(false)
            .min_width(800.0)
            .show(egui_context.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.separator();

                    ui.horizontal(|ui| {
                        let mut immune_system = immune_system.single_mut();
                        let age_factor = host.age / 300.0;

                        for levelup in selected.iter() {
                            image_button(ui, *levelup, || {
                                levelup.apply_immune_system(&mut immune_system, age_factor);
                                levelup.apply_host(&mut host, age_factor);
                                *levelups = None;
                                let _ = state.pop();
                                audio.play(
                                    audio_assets.improved.clone_weak(),
                                    PlaybackSettings {
                                        repeat: false,
                                        speed: 1.25,
                                        volume: 0.2,
                                    },
                                );
                            });
                        }
                    });

                    ui.add_space(10.0);
                });
            });
    } else {
        *levelups = Some(LevelUp::iter().choose_multiple(&mut rand::thread_rng(), 3));
    }
}

fn image_button(ui: &mut Ui, levelup: LevelUp, mut on_click: impl FnMut()) {
    ui.with_layout(Layout::left_to_right(), |ui| {
        ui.set_width(300.0);

        if ImageButton::new(
            egui::TextureId::User(levelup.to_image_id()),
            egui::vec2(48.0, 48.0),
        )
        .tint(Color32::WHITE)
        .ui(ui)
        .on_hover_text(levelup.info())
        .clicked()
        {
            on_click()
        }
        ui.small(&format!("{}", levelup));
    });
}

#[derive(EnumIter, Clone, Copy)]
pub enum LevelUp {
    Attack,
    Speed,
    TotalHealth,
    CurrentHealth,
    Regen,
    Dilatation,
}

impl fmt::Display for LevelUp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LevelUp::Attack => f.pad("Immune response"),
            LevelUp::Speed => f.pad("Blood flow"),
            LevelUp::TotalHealth => f.pad("Strength"),
            LevelUp::CurrentHealth => f.pad("Boost"),
            LevelUp::Regen => f.pad("Healing"),
            LevelUp::Dilatation => f.pad("Blood vessel"),
        }
    }
}

impl LevelUp {
    pub const fn to_image_id(self) -> u64 {
        match self {
            LevelUp::Speed => 9,
            LevelUp::Attack => 10,
            LevelUp::TotalHealth => 11,
            LevelUp::CurrentHealth => 12,
            LevelUp::Regen => 13,
            LevelUp::Dilatation => 14,
        }
    }

    pub fn info(&self) -> WidgetText {
        let mut layout = LayoutJob::default();
        match self {
            LevelUp::Attack => {
                layout.append(
                    "White cell production increase",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            LevelUp::Speed => {
                layout.append(
                    "Movement speed increase",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            LevelUp::TotalHealth => {
                layout.append(
                    "Total health increase",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            LevelUp::CurrentHealth => {
                layout.append(
                    "Restore current health",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            LevelUp::Regen => {
                layout.append(
                    "Healing increase",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            LevelUp::Dilatation => {
                layout.append(
                    "Increase blood vessel size",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
        }
        layout.into()
    }

    fn apply_immune_system(&self, immune_system: &mut ImmuneSystem, factor: f32) {
        match self {
            LevelUp::Attack => {
                immune_system.attack_spawn_rate += factor;
            }
            LevelUp::Speed => {
                immune_system.speed += 100.0 * factor;
            }
            LevelUp::TotalHealth => {
                immune_system.original_health += 150.0 * factor;
            }
            LevelUp::CurrentHealth => {
                immune_system.health = immune_system.original_health;
            }
            LevelUp::Regen => {}
            LevelUp::Dilatation => {}
        }
    }
    fn apply_host(&self, host: &mut HostState, factor: f32) {
        match self {
            LevelUp::Attack => {}
            LevelUp::Speed => {}
            LevelUp::TotalHealth => {}
            LevelUp::CurrentHealth => {}
            LevelUp::Regen => {
                host.regen += factor / 2.0;
            }
            LevelUp::Dilatation => {
                host.dilatation += factor * 400.0;
            }
        }
    }
}
