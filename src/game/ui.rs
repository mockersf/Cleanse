use bevy::prelude::*;
use bevy_egui::{
    egui::{self, text::LayoutJob, Color32, TextFormat, TextStyle, WidgetText},
    EguiContext,
};

use crate::GlobalState;

use super::{immune_system::ImmuneSystem, HostState};

pub fn status(
    mut egui_context: ResMut<EguiContext>,
    state: Res<HostState>,
    global_state: Res<GlobalState>,
    immune_system: Query<&ImmuneSystem>,
) {
    egui::TopBottomPanel::top("top").show(egui_context.ctx_mut(), |ui| {
        let mut status_to_display = LayoutJob::default();

        let healthbar_size = 144.0;

        let immune_system = immune_system.single();
        let current_health =
            (immune_system.health / immune_system.original_health * healthbar_size) as usize;
        status_to_display.append(
            &"-".repeat(current_health),
            0.0,
            TextFormat {
                color: Color32::GREEN,
                background: Color32::GREEN,
                style: TextStyle::Small,
                ..Default::default()
            },
        );
        status_to_display.append(
            &"-".repeat(healthbar_size as usize - current_health),
            0.0,
            TextFormat {
                color: Color32::RED,
                background: Color32::RED,
                style: TextStyle::Small,
                ..Default::default()
            },
        );

        status_to_display.append(
            "  -  ",
            0.0,
            TextFormat::simple(egui::TextStyle::Small, Color32::GRAY),
        );

        status_to_display.append(
            &format!("age  {: >5.1}", state.age),
            0.0,
            TextFormat::simple(egui::TextStyle::Monospace, Color32::WHITE),
        );

        status_to_display.append(
            "  -  ",
            0.0,
            TextFormat::simple(egui::TextStyle::Small, Color32::GRAY),
        );

        if global_state.expectancy > 0.0 {
            let color = if state.age > global_state.expectancy {
                Color32::GOLD
            } else {
                Color32::DARK_GREEN
            };
            let current_life =
                ((state.age / global_state.expectancy).min(1.0) * healthbar_size) as usize;
            status_to_display.append(
                &"-".repeat(current_life),
                0.0,
                TextFormat {
                    color,
                    background: color,
                    style: TextStyle::Small,
                    ..Default::default()
                },
            );
            status_to_display.append(
                &"-".repeat(healthbar_size as usize - current_life),
                0.0,
                TextFormat {
                    color: Color32::TRANSPARENT,
                    background: Color32::TRANSPARENT,
                    style: TextStyle::Small,
                    ..Default::default()
                },
            );
        }

        ui.label(WidgetText::LayoutJob(status_to_display));
    });
}
