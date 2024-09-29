use std::{borrow::{Borrow, BorrowMut}, ops::{Deref, DerefMut}};

use bevy::prelude::*;
use bevy_ascii_terminal::{StringFormatter, Terminal, TileFormatter};
use crate::{item::Item, player, CurrentRoom, InputCommandEvent, InputCommands, Player, Room, FORE_COLOR, GAMEBOARD_WIDTH, GAME_HEIGHT, GAME_WIDTH};

use super::common_components::*;


#[derive(Clone, Copy)]
pub struct InventoryItem {
    pub item: Entity,
    pub qty: i32,
}

#[derive(Component)]
pub struct Inventory {
    pub inventory: Vec<InventoryItem>,
}

pub fn process_inventory(mut terminal: Query<&mut Terminal>, mut message_event: EventReader<InputCommandEvent>, mut player: Query<(&Position, Option<&mut Inventory>, &CardinalDirection), With<Player>>, mut items: Query<(&Item, &Renderable, &mut Position), Without<Player>>, mut room: Query<(&Room, Option<&mut Inventory>), (With<CurrentRoom>, Without<Player>)>) {
    let mut term = terminal.get_single_mut().unwrap();
    let mut player_entity = player.get_single_mut().unwrap();
    
    for message in message_event.read() {
        
        if message.command == InputCommands::LIST_INVENTORY {

            if player_entity.1.is_some()  {
                term.clear_box(IVec2::new(GAMEBOARD_WIDTH+1, GAME_HEIGHT-14), IVec2::new(GAME_WIDTH, GAME_HEIGHT-13));
                term.put_string(IVec2::new(GAMEBOARD_WIDTH+1, GAME_HEIGHT-13), "- Inventory -".fg(FORE_COLOR));
                if !player_entity.1.as_mut().unwrap().inventory.is_empty() {
                    for inventory_item in player_entity.1.as_mut().unwrap().inventory.clone() {
                        let item_entity = items.get(inventory_item.item).unwrap();
                        term.put_char(IVec2::new(GAMEBOARD_WIDTH + 1, GAME_HEIGHT-14), item_entity.1.glyph.fg(item_entity.1.fg).bg(item_entity.1.bg));
                        term.put_string(IVec2::new(GAMEBOARD_WIDTH + 3, GAME_HEIGHT-14 ), (&item_entity.0.name).fg(FORE_COLOR));
                        println!("{}", item_entity.0.name);
                    }
                }
            }
        }

        if message.command == InputCommands::DROP_ITEM {
            let requested_item = message.data.clone();
            if player_entity.1.is_some() {
                if !player_entity.1.as_mut().unwrap().inventory.is_empty() {
                    let mut index = 0;
                    let mut dir = &CardDir::NORTH;
                    let mut item = &InventoryItem {item: Entity::from_raw(0), qty: 0};
                    for inventory_item in player_entity.1.as_mut().unwrap().inventory.iter().enumerate() {
                        let item_entity = items.get(inventory_item.1.item).unwrap();
                        if item_entity.0.name == requested_item.to_ascii_uppercase() {
                            dir = &player_entity.2.dir;
                            index = inventory_item.0;
                            item = inventory_item.1;
                        } else {
                            term.clear_box(IVec2::new (GAMEBOARD_WIDTH+1, GAME_HEIGHT-15), IVec2::new(GAME_WIDTH, GAME_HEIGHT- 14));
                            term.put_string(IVec2::new(GAMEBOARD_WIDTH+1, GAME_HEIGHT - 14), "You don't have".fg(FORE_COLOR));
                            term.put_string(IVec2::new(GAMEBOARD_WIDTH+1, GAME_HEIGHT - 15), format!("a {}", requested_item.to_ascii_uppercase()).fg(FORE_COLOR));
                        }
                    }
                    let mut selected_item = items.get_mut(item.item).unwrap();
                    match dir {
                        CardDir::NORTH => {
                            selected_item.2.x = player_entity.0.x;
                            selected_item.2.y = player_entity.0.y+1;
                        },
                        CardDir::SOUTH => {
                            selected_item.2.x = player_entity.0.x;
                            selected_item.2.y = player_entity.0.y-1;
                        },
                        CardDir::EAST => {
                            selected_item.2.x = player_entity.0.x+1;
                            selected_item.2.y = player_entity.0.y;
                        },
                        CardDir::WEST => {
                            selected_item.2.x = player_entity.0.x-1;
                            selected_item.2.y = player_entity.0.y;
                        }
                    }
                    // Assign room inventory at creation or when an item is dropped?
                    room.single_mut().1.as_mut().unwrap().inventory.push(InventoryItem{item: item.item , qty: 1});
                    player_entity.1.as_mut().unwrap().inventory.remove(index);
                } else {
                    term.clear_box(IVec2::new (GAMEBOARD_WIDTH+1, GAME_HEIGHT-15), IVec2::new(GAME_WIDTH, GAME_HEIGHT- 14));
                    term.put_string(IVec2::new(GAMEBOARD_WIDTH+1, GAME_HEIGHT - 14), "You don't have".fg(FORE_COLOR));
                    term.put_string(IVec2::new(GAMEBOARD_WIDTH+1, GAME_HEIGHT - 15), format!("a {}", requested_item.to_ascii_uppercase()).fg(FORE_COLOR));
                }
            }
        }
    } 
}
