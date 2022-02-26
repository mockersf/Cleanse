use bevy::{math::const_vec3, prelude::*};
use rand::Rng;

use crate::{tear_down, GameState};

#[derive(Component)]
struct ScreenTag;

#[derive(Default)]
struct Screen {
    done: Option<Timer>,
}

pub struct SplashPlugin;
impl bevy::app::Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Screen>()
            .add_system_set(SystemSet::on_enter(GameState::Splash).with_system(setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Splash).with_system(tear_down::<ScreenTag>),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Splash)
                    .with_system(check_finished)
                    .with_system(animate_logo),
            );
    }
}

fn setup(mut commands: Commands, mut screen: ResMut<Screen>, asset_server: Res<AssetServer>) {
    debug!("Loading Screen");

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("logo.png"),
            ..Default::default()
        })
        .insert_bundle((ScreenTag, SplashGiggle(Timer::from_seconds(0.05, true))));

    screen.done = Some(Timer::from_seconds(0.7, false));
}

#[derive(Component)]
struct SplashGiggle(Timer);

fn check_finished(
    time: Res<Time>,
    mut screen: ResMut<Screen>,
    mut state: ResMut<State<GameState>>,
) {
    if let Some(ref mut timer) = screen.done {
        timer.tick(time.delta());
        if timer.just_finished() {
            state.set(GameState::Menu).unwrap();
        }
    }
}

fn animate_logo(
    time: Res<Time>,
    mut query: Query<(&mut SplashGiggle, &mut Transform), With<ScreenTag>>,
) {
    for (mut timer, mut transform) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let translation = transform.translation;
            if translation.x != 0. || translation.y != 0. {
                *transform = Transform::identity();
                continue;
            }

            let scale = transform.scale;
            // `scale.0 != 1.` for floating numbers
            if (scale.x - 1.) > 0.01 {
                *transform = Transform::identity();
                continue;
            }

            let mut rng = rand::thread_rng();
            let act = rng.gen_range(0..100);

            if act < 20 {
                let span = 1.;
                let x: f32 = rng.gen_range(-span..span);
                let y: f32 = rng.gen_range(-span..span);
                *transform = Transform::from_translation(const_vec3!([x, y, 0.]));
            }
            if act > 80 {
                let scale_diff = 0.02;
                let new_scale: f32 = rng.gen_range((1. - scale_diff)..(1. + scale_diff));
                *transform = Transform::from_scale(Vec3::splat(new_scale));
            }
        }
    }
}
