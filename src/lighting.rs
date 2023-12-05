use bevy::prelude::*;

pub const LIGHTING_STARTING_DAYSTATE: DayState = DayState::Day;
pub const LIGHTING_DAY_LENGTH: u8 = 5;
pub const LIGHTING_NIGHT_LENGTH: u8 = 5;
pub const LIGHTING_TEMPORARY_TURN_TIME_SECONDS: f32 = 1.;

//Normalized version of (1.8, 1., 1.4)
pub const LIGHT_LOCATION: Vec3 = Vec3::new(0.722897, 0.40161, 0.562254);

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::rgb(1., 1., 1.),
            brightness: 0.1,
        })
        .add_systems(Startup, spawn_day_night_cycle_manager)
        .add_systems(Update, (temporary_timer_update, check_for_day_state_change));
    }
}

#[derive(Debug)]
pub enum DayState {
    Day,
    Night,
}

#[derive(Component, Debug)]
pub struct DayNightCycleManager {
    day_state: DayState,
    day_length: u8,
    night_length: u8,
    turns_since_last_day_state_change: u8,
    temporary_timer: Timer,
}

fn temporary_timer_update(mut query: Query<&mut DayNightCycleManager>, time: Res<Time>) {
    let day_night_cycle_manager = query.single_mut().into_inner();
    day_night_cycle_manager.temporary_timer.tick(time.delta());
    if day_night_cycle_manager.temporary_timer.just_finished() {
        day_night_cycle_manager.turns_since_last_day_state_change += 1;
    }
}

fn spawn_day_night_cycle_manager(mut commands: Commands) {
    commands
        .spawn((DayNightCycleManager {
            day_state: LIGHTING_STARTING_DAYSTATE,
            day_length: LIGHTING_DAY_LENGTH,
            night_length: LIGHTING_NIGHT_LENGTH,
            turns_since_last_day_state_change: 0,
            temporary_timer: Timer::from_seconds(
                LIGHTING_TEMPORARY_TURN_TIME_SECONDS,
                TimerMode::Repeating,
            ),
        }, Transform::default()))
        .with_children(|parent| {
            // parent.spawn(DirectionalLightBundle {
            //     transform: Transform::default()
            //         .looking_to(-Vec3::Y, Vec3::X),
            //     directional_light: DirectionalLight {
            //         color: Color::hex("FFFCE9").unwrap(),
            //         illuminance: 32000.,
            //         shadows_enabled: true,
            //         ..Default::default()
            //     },
            //     ..Default::default()
            // });
            // parent.spawn(DirectionalLightBundle {
            //     transform: Transform::from_translation(-LIGHT_LOCATION)
            //         .looking_at(Vec3::ZERO, Vec3::Y),
            //     directional_light: DirectionalLight {
            //         color: Color::hex("D1E8EF").unwrap(),
            //         illuminance: 0.1,
            //         shadows_enabled: true,
            //         ..Default::default()
            //     },
            //     ..Default::default()
            // });
        });
        commands.spawn(DirectionalLightBundle {
            transform: Transform::default()
                .looking_to(-Vec3::Y, Vec3::X),
            directional_light: DirectionalLight {
                color: Color::hex("FFFCE9").unwrap(),
                illuminance: 32000.,
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        });
}

fn check_for_day_state_change(mut query: Query<&mut DayNightCycleManager>) {
    let day_night_cycle_manager = query.single_mut().into_inner();
    let change_day_state_at_turn_count = {
        match day_night_cycle_manager.day_state {
            DayState::Day => day_night_cycle_manager.day_length,
            DayState::Night => day_night_cycle_manager.night_length,
        }
    };
    if day_night_cycle_manager.turns_since_last_day_state_change >= change_day_state_at_turn_count {
        change_day_state(day_night_cycle_manager);
    }
}

fn change_day_state(lighting_manager: &mut DayNightCycleManager) {
    lighting_manager.day_state = match lighting_manager.day_state {
        DayState::Day => DayState::Night,
        DayState::Night => DayState::Day,
    };
    lighting_manager.turns_since_last_day_state_change = 0;
}

fn slerp_sun_moon_locations(
    mut light_query: Query<(&mut Transform, &DirectionalLight, &Parent)>,
    manager_query: Query<(&DayNightCycleManager, &Children)>,
) {
    let (day_night_cycle_manager, manager_children) = manager_query.single();
    for (light_transform, directional_light, light_parent) in light_query.iter_mut() {

    }
}
