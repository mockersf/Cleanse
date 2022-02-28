use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Color32, Stroke, TextStyle},
    EguiContext,
};

use crate::GlobalState;

use super::{immune_system::ImmuneSystem, HostState};

pub fn status(
    mut egui_context: ResMut<EguiContext>,
    state: Res<HostState>,
    global_state: Res<GlobalState>,
    immune_system: Query<&ImmuneSystem>,
    mut healthbar_animation: Local<(Option<Timer>, f32)>,
    time: Res<Time>,
) {
    egui::TopBottomPanel::top("top").show(egui_context.ctx_mut(), |ui| -> egui::Rect {
        let (rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click());
        if ui.is_rect_visible(rect) {
            let radius = 0.2 * rect.height();

            {
                let mut rect = rect;
                rect.set_right(egui::lerp(rect.left()..=rect.right(), 0.45));

                let immune_system = immune_system.single();
                let current_health = immune_system.health / immune_system.original_health;
                if let Some(ref mut timer) = healthbar_animation.0 {
                    if timer.tick(time.delta()).finished() {
                        healthbar_animation.0 = None;
                        healthbar_animation.1 = current_health;
                    }
                }

                ui.painter()
                    .rect(rect, radius, Color32::RED, Stroke::none());
                let current_health = if current_health != healthbar_animation.1 {
                    if let Some(timer) = healthbar_animation.0.as_ref() {
                        egui::lerp(current_health..=healthbar_animation.1, timer.percent_left())
                    } else {
                        healthbar_animation.0 = Some(Timer::from_seconds(0.2, false));
                        healthbar_animation.1
                    }
                } else {
                    current_health
                };
                if current_health > 0.0 {
                    let end = egui::lerp(rect.left()..=rect.right(), current_health);
                    let mut health_bar = rect;
                    health_bar.set_right(end);
                    ui.painter()
                        .rect(health_bar, radius, Color32::GREEN, Stroke::none());
                }
            }

            {
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    &format!("Age: {:.1}", state.age),
                    TextStyle::Small,
                    Color32::WHITE,
                );
            }

            {
                if global_state.expectancy > 0.0 {
                    let mut rect = rect;
                    rect.set_left(egui::lerp(rect.left()..=rect.right(), 0.55));

                    let current_life = (state.age / global_state.expectancy).min(1.0);

                    if current_life < 1.0 {
                        ui.painter()
                            .rect(rect, radius, Color32::DARK_RED, Stroke::none());
                        let end = egui::lerp(rect.left()..=rect.right(), current_life);
                        let mut health_bar = rect;
                        health_bar.set_right(end);
                        ui.painter()
                            .rect(health_bar, radius, Color32::BLUE, Stroke::none());
                    } else {
                        ui.painter()
                            .rect(rect, radius, Color32::GOLD, Stroke::none());
                        let center = egui::lerp(
                            rect.left()..=rect.right(),
                            global_state.expectancy / state.age,
                        );
                        let mut last_max = rect;
                        last_max.set_left(center * 0.999);
                        last_max.set_right(center * 1.001);
                        ui.painter()
                            .rect(last_max, radius, Color32::DARK_RED, Stroke::none());
                    }
                }
            }
        }

        rect
    });
}
