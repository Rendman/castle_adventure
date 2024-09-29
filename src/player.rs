use std::borrow::BorrowMut;

use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy_ascii_terminal::{StringFormatter, Terminal, TileFormatter};

use crate::inventory::Inventory;
use crate::item::Item;
use crate::{can_enter_space, handle_player_typed_input, CurrentRoom, Message, Room, TypedInput};

use super::{FORE_COLOR, GAME_HEIGHT, GAMEBOARD_HEIGHT, GAMEBOARD_WIDTH};

use super::common_components::*;

#[derive(PartialEq)]
pub enum InputCommands {
    LIST_INVENTORY,
    INVALID_INPUT,
    DROP_ITEM,
}

#[derive(Event)]
pub struct InputCommandEvent {
    pub command: InputCommands,
    pub data: String,
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    pos: Position,
    visual: Renderable,
    player: Player,
    inventory: Inventory,
    facing: CardinalDirection,
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn(
        PlayerBundle{
            pos: Position{x: 5, y: 10},
            visual: Renderable{
                glyph: '♣',
                fg: FORE_COLOR,
                bg: Color::BLACK
            },
            player: Player,
            inventory: Inventory {inventory: Vec::new() },
            facing: CardinalDirection {dir: CardDir::NORTH},
    });
}

pub fn render_player(query: Query<(&Renderable, &Position, &CardinalDirection), With<Player>>, items: Query<(&Position, &Renderable, &Item)>, mut terminal: Query<&mut Terminal>) {
    let mut term = terminal.get_single_mut().unwrap();
    let player = query.get_single().unwrap();
    if player.1.x < 0 || player.1.x > GAMEBOARD_WIDTH - 1 || player.1.y < GAME_HEIGHT - GAMEBOARD_HEIGHT || player.1.y > GAME_HEIGHT-1 {
        return
    }

    term.put_char(IVec2::new(player.1.x, player.1.y), player.0.glyph.fg(player.0.fg).bg(player.0.bg));
    
}

pub fn handle_room_player_input(mut player: Query<(&mut Position,  &mut CardinalDirection, Option<&Inventory>), With<Player>>, roomdata: Query<(Entity, &Room), With<CurrentRoom>>, mut keycode: EventReader<KeyboardInput>, mut input_string: ResMut<TypedInput>, mut input_event: EventWriter<InputCommandEvent>) {

    let mut player = player.get_single_mut().unwrap();
    let mut position = player.0;
        
    let mut x = position.x;
    let mut y = position.y;

    let mut enter_pressed = false;

    for event in keycode.read() {

        if event.state == ButtonState::Released {
            continue;
        }

        match &event.logical_key {
            Key::ArrowUp => {
                y += 1;
                player.1.dir = CardDir::NORTH;
            },
            Key::ArrowDown => {
                y -= 1;
                player.1.dir = CardDir::SOUTH;
            },
            Key::ArrowLeft => {
                x -= 1;
                player.1.dir = CardDir::WEST;
            },
            Key::ArrowRight => {
                x += 1;
                player.1.dir = CardDir::EAST;
            },
            Key::Enter => {
                enter_pressed = true;
            }
            Key::Backspace => {
                input_string.input.pop();
            }
            Key::Space => {
                input_string.input.push(' ');
            }

            Key::Character(input) => {
                if input_string.input.len() >= 14 {
                    continue;
                }
                input_string.input.push_str(&input);
            }
            _ => {}
        }
    }
    
    if enter_pressed {
        handle_player_typed_input(input_string.input.clone(), input_event);
        enter_pressed = false;
        input_string.input.clear();

    }
    

    if can_enter_space(IVec2::new(x, y-(GAME_HEIGHT-GAMEBOARD_HEIGHT)), roomdata.single().1) {
        position.x = x;
        position.y = y;

    }    
    
}

