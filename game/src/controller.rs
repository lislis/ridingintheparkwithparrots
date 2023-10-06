use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy::math::Vec3Swizzles;
use rand::prelude::Rng;

use crate::*;

pub const CONTROLLER_IP: &str = "http://192.168.178.147/";

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Measurement>()
        .add_event::<MeasureEvent>()
        .add_systems(OnEnter(GameState::Loading), init_measurements)
        .add_systems(Update, poll_measurements.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, handle_measurements.run_if(in_state(GameState::Gameplay)));
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Measurement {
    pub request_timer: Timer,
    //pub current: f32
}

#[derive(Event)]
pub enum MeasureEvent {
    Left,
    Right,
    None,
}

fn init_measurements(
    mut commands: Commands,
) {
    commands.spawn(Measurement { 
        request_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        //current: 0.0
    });
}

fn poll_measurements(
    mut commands: Commands,
    time: Res<Time>,
    mut measure_q: Query<&mut Measurement>,
) {
    if let Ok(mut measurement) = measure_q.get_single_mut() {
        measurement.request_timer.tick(time.delta());

        if measurement.request_timer.just_finished() {
            if let Ok(url) = CONTROLLER_IP.try_into() {
                let req = reqwest::Request::new(reqwest::Method::GET, url);
                let req = ReqwestRequest::new(req);
                commands.spawn(req);
            }
        }
    } 
}

fn handle_measurements(
    mut commands: Commands, 
    //mut measure_q: Query<&mut Measurement>,
    results: Query<(Entity, &ReqwestBytesResult)>,
    mut measurement_event_writer: EventWriter<MeasureEvent>
) {
    //if let Ok(mut measurement) = measure_q.get_single_mut() {
        for (e, res) in results.iter() {
            let string = res.as_str().unwrap();
            // let measurements = string.split(",")
            //     .map(|x| x.parse::<f32>().unwrap_or_else(|_| 0.0))
            //     .collect::<Vec<f32>>();
            info!("{:?}", string);

            match string {
                "left" => measurement_event_writer.send(MeasureEvent::Left),
                "right" => measurement_event_writer.send(MeasureEvent::Right),
                _ => measurement_event_writer.send(MeasureEvent::None)
            }

            //bevy::log::info!("{} - {} - {}", x, y, z);
            //measurement.current = measurements[1];
            //measurement_event_writer.send(MeasureEvent { value: measurements[1] });
            commands.entity(e).despawn_recursive();
        }
   // }
    
}
