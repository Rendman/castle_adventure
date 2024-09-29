use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;
use bevy_ascii_terminal::{StringFormatter, Terminal, TileFormatter};
use serde_json::Value;

use crate::inventory::{Inventory, InventoryItem};
use crate::{Player, RoomChanged};

use super::item::{Item, ItemBundle};
use super::common_components::*;

use super::{GAMEBOARD_HEIGHT, GAMEBOARD_WIDTH, GAME_HEIGHT, FORE_COLOR};

//#[derive(Clone, Copy)]
pub enum ExitEdge {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    NONE
}

#[derive(Component)]
pub struct CurrentRoom {}

//#[derive(Clone, Copy)]
pub struct ExitArea {
    pub edge: ExitEdge,
    pub location: IVec2,
    pub exit_location: IVec2,
    pub next_room: Entity
}

#[derive(Component)]
pub struct Exits {
    pub exits: Vec<ExitArea>,
}

#[derive(Component)]
pub struct Room {
    pub room_name: String,
    pub room_data: Vec<char>,
    pub room_desc: String
}

pub fn load_rooms(mut commands: Commands) {
    // Super kludgy...
    // Build a HashMap to link all entities together that are referenced from each other
    let mut ref_map = HashMap::new();


    let data: String = fs::read_to_string("data.json").unwrap();
    let json_data: Value = serde_json::from_str(&data).unwrap();
    // First get the number of rooms.
    let num_rooms = json_data["rooms"].as_array().unwrap().len();

    // We have to read all rooms before we start to work the exits.

    for room in 0..num_rooms {
        let room_data: Vec<char> = json_data["rooms"][room]["room_data"].as_str().unwrap().to_string().replace("'","").replace(",", "").chars().collect();
        let room_name = json_data["rooms"][room]["room_name"].as_str().unwrap().to_string();
        let room_desc = json_data["rooms"][room]["room_desc"].as_str().unwrap().to_string();
        
        let room_st = Room {
            room_name: room_name.clone(),
            room_data: room_data,
            room_desc: room_desc,
        };
        let id = commands.spawn(room_st).id();
        ref_map.insert(room_name, id);
    }

    for room in 0..num_rooms {
        let room_name = json_data["rooms"][room]["room_name"].as_str().unwrap().to_string();
        let mut exits = Vec::new();

        let num_exits = json_data["rooms"][room]["exits"].as_array().unwrap().len();
        
        if num_exits == 0 {
            let exit_st = ExitArea {
                edge: ExitEdge::NONE,
                next_room: Entity::from_raw(99999),
                location: IVec2::new(0,0),
                exit_location: IVec2::new(0,0)
            };
            exits.push(exit_st);
        }

        for exit in 0..num_exits {
            if json_data["rooms"][room]["exits"][exit]["edge"] != Value::Null {
                let edge_sel = match json_data["rooms"][room]["exits"][exit]["edge"].as_str().unwrap().to_string().as_str() {
                    "NORTH" => ExitEdge::NORTH,
                    "SOUTH" => ExitEdge::SOUTH,
                    "EAST" => ExitEdge::EAST,
                    "WEST" => ExitEdge::WEST,
                    _ => ExitEdge::NONE,
                };

                let room_entity = ref_map.get(json_data["rooms"][room]["exits"][exit]["next_room"].as_str().unwrap()).unwrap();

                let exit_st = ExitArea {
                    edge: edge_sel,
                    next_room: *room_entity,
                    location: IVec2::new(0,0),
                    exit_location: IVec2::new(0,0)
                };

                exits.push(exit_st);
            }
            if json_data["rooms"][room]["exits"][exit]["edge"] == Value::Null {
                
                let room_entity = ref_map.get(json_data["rooms"][room]["exits"][exit]["next_room"].as_str().unwrap()).unwrap();

                let exit_st = ExitArea {
                    edge: ExitEdge::NONE,
                    next_room: *room_entity,
                    location: IVec2::new(0,0),
                    exit_location: IVec2::new(0,0)
                };

                exits.push(exit_st);
            }
        }
        commands.entity(*ref_map.get(&room_name).unwrap()).insert( Exits {exits: exits});
        commands.entity(*ref_map.get(&room_name).unwrap()).insert( Inventory {inventory: Vec::new()});
    }
    
    

    // Add some test items
    let testItem = ItemBundle {
        item: Item {name: "SWORD".to_string(), visible: true},
        visual: Renderable { glyph: '┼', fg: FORE_COLOR, bg: Color::BLACK },
        pos: Position { x: 15, y: 10 },
    };

    let testItem2 = ItemBundle {
        item: Item {name: "GEM".to_string(), visible: true},
        visual: Renderable {glyph: '♦', fg: FORE_COLOR, bg: Color::BLACK },
        pos: Position {x: 10, y: 15},
    };

    let mut itemVec = Vec::new();
    let mut itemVec2 = Vec::new();
    let itemID = commands.spawn(testItem).id();
    let itemID2 = commands.spawn(testItem2).id();
    let itemEntry = InventoryItem {item: itemID, qty: 1};
    itemVec.push(itemEntry);
    let itemEntry2 = InventoryItem {item: itemID2, qty: 1};
    itemVec2.push(itemEntry2);

    commands.entity(*ref_map.get("Courtyard").unwrap()).insert(CurrentRoom {});
    commands.entity(*ref_map.get("East Ballroom").unwrap()).insert(Inventory {inventory: itemVec} );
    commands.entity(*ref_map.get("Courtyard").unwrap()).insert(Inventory {inventory: itemVec2});

}


pub fn render_room(room: Query<&Room, With<CurrentRoom>>, mut terminal: Query<&mut Terminal>, items: Query<(&Room, &Item)>, mut room_changed: ResMut<RoomChanged>) {
    let mut term = terminal.get_single_mut().unwrap();
       
    for x in 0..GAMEBOARD_WIDTH+1 {
        term.put_char([x, (GAME_HEIGHT-1) - GAMEBOARD_HEIGHT], '─'.fg(FORE_COLOR));
    }

    for y in GAME_HEIGHT - GAMEBOARD_HEIGHT..GAME_HEIGHT {
        term.put_char([GAMEBOARD_WIDTH, y], '│'.fg(FORE_COLOR));
    }
    term.put_char([GAMEBOARD_WIDTH, (GAME_HEIGHT - GAMEBOARD_HEIGHT)-1], '┘');

    let current_room = room.get_single().unwrap();

    for tile in current_room.room_data.iter().enumerate() {
        let offset = IVec2::new(0,(GAME_HEIGHT) - GAMEBOARD_HEIGHT);
        let pos = IVec2::new(tile.0 as i32 % GAMEBOARD_WIDTH, tile.0 as i32 / GAMEBOARD_WIDTH);
        term.put_char(pos+offset, tile.1.fg(FORE_COLOR));
    }

    let line_count = current_room.room_desc.matches("\n").count();

    if room_changed.changed {
        term.clear();
        // display location information
        room_changed.changed = false;
        term.put_string([0, 5-line_count], format!("{}", current_room.room_desc).fg(FORE_COLOR));
    }

}

pub fn is_player_on_exit(mut commands: Commands, mut location: Query<&mut Position, With<Player>>, roomdata: Query<(Entity, &Room), With<CurrentRoom>>, roomexits: Query<&Exits, With<CurrentRoom>>, mut room_changed: ResMut<RoomChanged>) {
    let loc = IVec2::new(location.get_single().unwrap().x, location.get_single_mut().unwrap().y);
    let mut pos = location.get_single_mut().unwrap();    

    for exit in &roomexits.get_single().unwrap().exits {  
        match exit.edge {
            ExitEdge::SOUTH => {
                if loc.y < (GAME_HEIGHT - GAMEBOARD_HEIGHT) {
                    pos.y = GAME_HEIGHT-1;
                    commands.entity(exit.next_room).insert(CurrentRoom {});
                    commands.entity(roomdata.single().0).remove::<CurrentRoom>();
                    room_changed.changed = true;

                }
            }
            ExitEdge::NORTH => {
                if loc.y > GAME_HEIGHT-1 {
                    pos.y = GAME_HEIGHT - GAMEBOARD_HEIGHT;
                    commands.entity(exit.next_room).insert(CurrentRoom {});
                    commands.entity(roomdata.single().0).remove::<CurrentRoom>();
                    room_changed.changed = true;

                }
            }
            ExitEdge::EAST => {
                if loc.x > GAMEBOARD_WIDTH-1 {
                    pos.x = 0;
                    commands.entity(exit.next_room).insert(CurrentRoom {});
                    commands.entity(roomdata.single().0).remove::<CurrentRoom>();
                    room_changed.changed = true;

                }
            }
            ExitEdge::WEST => {
                if loc.x < 0 {
                    pos.x = GAMEBOARD_WIDTH-1;
                    commands.entity(exit.next_room).insert(CurrentRoom {});
                    commands.entity(roomdata.single().0).remove::<CurrentRoom>();
                    room_changed.changed = true;

                }
            }
            ExitEdge::NONE => {

            }
        }
        
        if loc == exit.location {
            commands.entity(exit.next_room).insert(CurrentRoom {});
            commands.entity(roomdata.single().0).remove::<CurrentRoom>();
            room_changed.changed = true;

        }
    }
}

pub fn can_enter_space(player_location: IVec2, roomdata: &Room) -> bool {
    let glyph_index: usize = ((player_location.y * GAMEBOARD_WIDTH) + player_location.x) as usize;
    if player_location.x < 0 || player_location.x > GAMEBOARD_WIDTH-1 || player_location.y < (0) || player_location.y > GAMEBOARD_HEIGHT-1 {
        return true;
    }
    if !(glyph_index > 432) {
        if roomdata.room_data[glyph_index] == '▓' {
            return false;
        }
    }
    true
}