use bevy::{prelude::*, time::Stopwatch};
use bevy_ascii_terminal::prelude::*;

use inventory::process_inventory;
use item::{is_player_on_item, render_items};
use player::*;
use rooms::*;
use status_area::*;
use title_instruction_screen::*;

mod common_components;
mod player;
mod title_instruction_screen;
mod rooms;
mod item;
mod inventory;
mod status_area;

const GAME_WIDTH: i32= 40;
const GAME_HEIGHT: i32 = 25;

const GAMEBOARD_WIDTH: i32 = 24;
const GAMEBOARD_HEIGHT: i32 = 18;

const FORE_COLOR: Color = Color::rgb(0.667, 0.667, 0.667);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
   
    TitleScreen,
    InstructionScreen,
    #[default]
    InGame,
}

#[derive(Resource)]
pub struct TypedInput {
    input: String,
}

#[derive(Event)]
pub struct Message {
    msg: String,
}

pub struct MessageEntry {
    msg: String,
    time: Timer
}

#[derive(Resource)]
pub struct MessageLog {
    messages: Vec<MessageEntry>,
}

#[derive(Resource)]
pub struct RoomChanged {
    changed: bool
}

fn setup(mut commands: Commands) {
    // Create the terminal
    let terminal = Terminal::new([GAME_WIDTH,GAME_HEIGHT]);
    commands.spawn((
        // Spawn the terminal bundle from our terminal
        TerminalBundle::from(terminal),
        // Automatically set up the camera to render the terminal
        AutoCamera,
    ));

    commands.insert_resource(TypedInput {input: String::new() });
    commands.insert_resource(MessageLog {messages: Vec::new()});
    commands.insert_resource(RoomChanged {changed: true });
}

fn main () {
    App::new()
    .add_plugins((DefaultPlugins, TerminalPlugin))
    .init_state::<GameState>()

    .add_systems(OnEnter(GameState::TitleScreen), setup_title_screen)
    .add_systems(OnExit(GameState::TitleScreen), cleanup_title_screen)
    .add_systems(Update, title_screen.run_if(in_state(GameState::TitleScreen)))

    .add_systems(OnEnter(GameState::InstructionScreen), setup_instruction_screen)
    .add_systems(OnExit(GameState::InstructionScreen), cleanup_instruction_screen)
    .add_systems(Update, instruction_screen.run_if(in_state(GameState::InstructionScreen)))


    // In Game
    .add_systems(OnEnter(GameState::InGame), (spawn_player, load_rooms.before(spawn_player)))
    .add_systems(Update, (handle_room_player_input.before(render_room), render_room, render_typed_input.after(render_room), render_messages.after(render_room), render_items.after(render_room), render_player.after(render_room)).run_if(in_state(GameState::InGame)))
    .add_systems(Update, (is_player_on_exit, is_player_on_item).before(render_room).run_if(in_state(GameState::InGame)))
    .add_systems(Update, (process_inventory).run_if(in_state(GameState::InGame)))

    .add_systems(Startup, setup)

    .add_event::<Message>()
    .add_event::<InputCommandEvent>()

    .run();
}

