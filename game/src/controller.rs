use crate::*;

pub const SERIAL_PORT: &str = "/dev/ttyUSB0";
//pub const SERIAL_PORT: &str = "/dev/ttyACM0";

#[derive(PartialEq, Debug, Reflect)]
pub enum Dir {
    Left,
    Right,
    Neutral
}

impl Default for Dir {
    fn default() -> Self {Self::Neutral}
}

#[derive(Debug, PartialEq, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Movement {
    pub value: f32,
    pub direction: Dir,
    //moving_back: bool
}


pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Movement>()
        .add_plugins(SerialPlugin::new(SERIAL_PORT, 115200))
        .add_systems(OnEnter(GameState::Gameplay), init_movement)
        .add_systems(OnExit(GameState::Gameplay), rm_movement)
        .add_systems(Update, read_serial.run_if(in_state(GameState::Gameplay)));
    }
}

impl Movement {
    fn new() -> Self {
        Self {
            value: 0.0,
            direction: Dir::Neutral,
        }
    }
    fn update(&mut self, value: f32) {
        self.value = value;
        let new_direction = match value {
            x if x < -0.1 => Dir::Right,
            x if x > 0.1 => Dir::Left,
            _ => Dir::Neutral
        };
        
        self.direction = new_direction;
    }
}

fn init_movement(mut commands: Commands) {
    commands.spawn((
        Movement::new(),
        Name::new("Movement")
    ));
}

fn rm_movement(
    mut commands: Commands, 
    movement_q: Query<(Entity, &Movement)>
) {
    let (entity, _movement) = movement_q.single();
    commands.entity(entity).despawn_recursive();
}

fn read_serial(
    mut ev_serial: EventReader<SerialReadEvent>,
    mut movement_q: Query<&mut Movement>,
) {
    let mut movement = movement_q.single_mut();
    // you can get label of the port and received data buffer from `SerialReadEvent`
    for SerialReadEvent(label, buffer) in ev_serial.iter() {
        let s = String::from_utf8(buffer.clone()).unwrap();
        
        let new_val = s.trim().parse::<f32>().unwrap_or_default();
        //println!("received packet from {label}: {s}, ({new_val})");
        movement.update(new_val);
    }
}
