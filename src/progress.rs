use std::fmt::{self, Formatter};

use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, text::LayoutJob, Align2, Color32, ImageButton, Layout, RichText, TextFormat, Ui,
        Widget, WidgetText,
    },
    EguiContext,
};
use strum::EnumIter;

use crate::{menu::button, GameState, GlobalState};

pub struct ProgressPlugin;

impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_update(GameState::Progress).with_system(progress));
    }
}

#[derive(EnumIter)]
pub enum Progress {
    Disinfectant,
    Antibiotics,
    Vaccine,
    Sanitation,
    PersonalHygiene,
    PreventiveMeasures,
    SickDays,
    FreeHealthcare,
    ParentalLeave,
    LevelUpSpeed,
    LevelUpAttack,
    LevelUpHealth,
}

impl fmt::Display for Progress {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Progress::Disinfectant => f.pad("Disinfectant"),
            Progress::Antibiotics => f.pad("Antibiotics"),
            Progress::Vaccine => f.pad("Vaccine"),
            Progress::Sanitation => f.pad("Sanitation"),
            Progress::PersonalHygiene => f.pad("Personal Hygiene"),
            Progress::PreventiveMeasures => f.pad("Preventive Measures"),
            Progress::SickDays => f.pad("Sick Days"),
            Progress::FreeHealthcare => f.pad("Free Healthcare"),
            Progress::ParentalLeave => f.pad("Parental Leave"),
            Progress::LevelUpSpeed => f.pad("Blood flow"),
            Progress::LevelUpAttack => f.pad("Immune response"),
            Progress::LevelUpHealth => f.pad("Resistance"),
        }
    }
}

impl Progress {
    pub const fn to_image_id(&self) -> u64 {
        match self {
            Progress::Disinfectant => 0,
            Progress::Antibiotics => 1,
            Progress::Vaccine => 2,
            Progress::Sanitation => 3,
            Progress::PersonalHygiene => 4,
            Progress::PreventiveMeasures => 5,
            Progress::SickDays => 6,
            Progress::FreeHealthcare => 7,
            Progress::ParentalLeave => 8,
            Progress::LevelUpSpeed => 9,
            Progress::LevelUpAttack => 10,
            Progress::LevelUpHealth => 11,
        }
    }

    pub fn details(&self) -> WidgetText {
        let mut layout = LayoutJob::default();
        match self {
            Progress::Disinfectant => {
                layout.append(
                    "Bacteria risk reduction",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nVirus risk reduction",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            Progress::Antibiotics => {
                layout.append(
                    "Healing improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            Progress::Vaccine => {
                layout.append(
                    "Large virus risk reduction",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            Progress::Sanitation => {
                layout.append(
                    "Bacteria risk reduction",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nVirus risk reduction",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nHealing improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nResistance improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            Progress::PersonalHygiene => {
                layout.append(
                    "Bacteria risk reduction",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nVirus risk reduction",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nHealing improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nBlood flow improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            Progress::PreventiveMeasures => {
                layout.append(
                    "Bacteria risk reduction",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nVirus risk reduction",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nResistance improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nBlood flow improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            Progress::SickDays => {
                layout.append(
                    "Bacteria risk reduction",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nVirus risk reduction",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nHealing improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nVessel improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            Progress::FreeHealthcare => {
                layout.append(
                    "Cancer risk reduction",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nImmune response improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nHealing improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nVessel improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            Progress::ParentalLeave => {
                layout.append(
                    "Cancer risk reduction",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nLarge immune response improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
                layout.append(
                    "\nHealing improvement",
                    0.0,
                    TextFormat::simple(egui::TextStyle::Small, Color32::LIGHT_GRAY),
                );
            }
            Progress::LevelUpSpeed => {}
            Progress::LevelUpAttack => {}
            Progress::LevelUpHealth => {}
        }
        layout.into()
    }
}

impl Progress {
    fn base_and_factor(&self) -> (usize, usize) {
        match self {
            Progress::Disinfectant => (25, 1),
            Progress::Antibiotics => (25, 2),
            Progress::Vaccine => (25, 5),
            Progress::PersonalHygiene => (50, 5),
            Progress::Sanitation => (50, 10),
            Progress::PreventiveMeasures => (50, 15),
            Progress::SickDays => (100, 5),
            Progress::FreeHealthcare => (100, 10),
            Progress::ParentalLeave => (100, 20),
            _ => (0, 0),
        }
    }
}

#[derive(Default)]
pub struct Effect {
    pub dilatation: f32,
    pub cancer: f32,
    pub bacteria: f32,
    pub virus: f32,
    pub regen: f32,
    pub health: f32,
    pub speed: f32,
    pub attack: f32,
}

impl Effect {
    pub fn apply(&mut self, progress: Progress) {
        match progress {
            Progress::Disinfectant => {
                self.bacteria -= 0.2;
                self.virus -= 0.2;
            }
            Progress::Antibiotics => {
                self.regen += 0.3;
            }
            Progress::Vaccine => {
                self.virus -= 0.5;
            }
            Progress::PersonalHygiene => {
                self.bacteria -= 0.2;
                self.virus -= 0.2;
                self.regen += 0.15;
                self.health += 15.0;
            }
            Progress::Sanitation => {
                self.bacteria -= 0.4;
                self.virus -= 0.2;
                self.regen += 0.15;
                self.speed += 10.0;
            }
            Progress::PreventiveMeasures => {
                self.bacteria -= 0.2;
                self.virus -= 0.2;
                self.health += 10.0;
                self.speed += 5.0;
            }
            Progress::SickDays => {
                self.bacteria -= 0.15;
                self.virus -= 0.1;
                self.dilatation += 200.0;
                self.regen += 0.15;
            }
            Progress::FreeHealthcare => {
                self.dilatation += 200.0;
                self.cancer -= 0.025;
                self.attack += 0.2;
                self.regen += 0.1;
            }
            Progress::ParentalLeave => {
                self.attack += 0.4;
                self.cancer -= 0.015;
                self.regen += 0.1;
            }
            Progress::LevelUpSpeed => (),
            Progress::LevelUpAttack => (),
            Progress::LevelUpHealth => (),
        }
    }
}

impl GlobalState {
    pub fn has(&self, progress: &Progress) -> bool {
        match progress {
            Progress::Disinfectant => self.disinfectant != usize::MAX,
            Progress::Antibiotics => self.antibiotics != usize::MAX,
            Progress::Vaccine => self.vaccine != usize::MAX,
            Progress::Sanitation => self.sanitation != usize::MAX,
            Progress::PersonalHygiene => self.personal_hygiene != usize::MAX,
            Progress::PreventiveMeasures => self.preventive_measures != usize::MAX,
            Progress::SickDays => self.sick_days != usize::MAX,
            Progress::FreeHealthcare => self.free_healthcare != usize::MAX,
            Progress::ParentalLeave => self.parental_leave != usize::MAX,
            _ => false,
        }
    }

    fn get(&mut self, progress: &Progress) {
        match progress {
            Progress::Disinfectant => self.disinfectant = self.generation,
            Progress::Antibiotics => self.antibiotics = self.generation,
            Progress::Vaccine => self.vaccine = self.generation,
            Progress::Sanitation => self.sanitation = self.generation,
            Progress::PersonalHygiene => self.personal_hygiene = self.generation,
            Progress::PreventiveMeasures => self.preventive_measures = self.generation,
            Progress::SickDays => self.sick_days = self.generation,
            Progress::FreeHealthcare => self.free_healthcare = self.generation,
            Progress::ParentalLeave => self.parental_leave = self.generation,
            _ => (),
        }
    }

    fn cost(&self, progress: &Progress) -> usize {
        progress.base_and_factor().0
            + progress.base_and_factor().1 * (self.current_progress_multiplier() * 2 / 3 + 2)
    }
}

fn progress(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    mut global_state: ResMut<GlobalState>,
) {
    egui::Window::new(RichText::new("Cleanse").color(Color32::RED))
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .min_width(800.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.strong(format!(
                    "Progress Points: {:.0}",
                    global_state.progress.floor()
                ));
                ui.separator();

                ui.horizontal(|ui| {
                    image_button(ui, Progress::Disinfectant, &mut *global_state);
                    image_button(ui, Progress::Antibiotics, &mut *global_state);
                    image_button(ui, Progress::Vaccine, &mut *global_state);
                });

                ui.horizontal(|ui| {
                    image_button(ui, Progress::PersonalHygiene, &mut *global_state);
                    image_button(ui, Progress::Sanitation, &mut *global_state);
                    image_button(ui, Progress::PreventiveMeasures, &mut *global_state);
                });

                ui.horizontal(|ui| {
                    image_button(ui, Progress::SickDays, &mut *global_state);
                    image_button(ui, Progress::FreeHealthcare, &mut *global_state);
                    image_button(ui, Progress::ParentalLeave, &mut *global_state);
                });

                ui.add_space(20.0);
                ui.vertical_centered(|ui| {
                    ui.set_width(350.);
                    button(
                        ui,
                        "Back",
                        || {
                            let _ = state.set(GameState::Menu);
                        },
                        true,
                        false,
                    );
                });
                ui.add_space(10.0);
            });
        });
}

fn image_button(ui: &mut Ui, progress: Progress, global_state: &mut GlobalState) {
    ui.with_layout(Layout::left_to_right(), |ui| {
        ui.set_width(300.0);

        let cost = global_state.cost(&progress);

        if match (global_state.has(&progress), cost) {
            (true, _) => true,
            (_, x) if (x as f32) > global_state.progress => true,
            _ => false,
        } {
            ui.set_enabled(false);
        }

        if ImageButton::new(
            egui::TextureId::User(progress.to_image_id()),
            egui::vec2(48.0, 48.0),
        )
        .tint(match (global_state.has(&progress), cost) {
            (true, _) => Color32::DARK_GREEN,
            (_, x) if (x as f32) > global_state.progress => Color32::DARK_GRAY,
            _ => Color32::WHITE,
        })
        .ui(ui)
        .on_hover_text(progress.details())
        .on_disabled_hover_text(progress.details())
        .clicked()
        {
            global_state.progress -= cost as f32;
            global_state.get(&progress);
        }
        if global_state.has(&progress) {
            ui.small(&format!("{:<20}\n\n", progress));
        } else {
            ui.small(&format!("{:<20}\n\n{}", progress, cost));
        }
    });
}
