use bevy::prelude::*;
use bevy_ascii_terminal::StringFormatter;
use bevy_ascii_terminal::Terminal;
use bevy_ascii_terminal::TileFormatter;

use crate::InputCommandEvent;
use crate::InputCommands;
use crate::MessageEntry;
use crate::MessageLog;
use crate::RoomChanged;
use crate::TypedInput;
use crate::GAMEBOARD_WIDTH;
use crate::FORE_COLOR;
use crate::GAME_HEIGHT;
use crate::GAME_WIDTH;



pub fn render_typed_input(mut terminal: Query<&mut Terminal>, input_string: Res<TypedInput>) {
    let mut term = terminal.get_single_mut().unwrap();

    // Do blinky prompt here

    term.clear_box(IVec2::new(GAMEBOARD_WIDTH+2, 2), IVec2::new(14, 2));
    // draw prompt
    term.put_char(IVec2::new(GAMEBOARD_WIDTH+1, 2), '?'.fg(FORE_COLOR));
    // draw input
    term.put_string(IVec2::new(GAMEBOARD_WIDTH+2, 2), (&input_string.input).fg(FORE_COLOR));
}

pub fn render_messages(mut terminal: Query<&mut Terminal>, mut message_event: EventReader<InputCommandEvent>, mut message_log: ResMut<MessageLog>, time: Res<Time>, room_changed: Res<RoomChanged>) {
    let mut term = terminal.get_single_mut().unwrap();
    
    if room_changed.changed {
        term.clear_box(IVec2::new(GAMEBOARD_WIDTH+2, 0), IVec2::new(GAME_WIDTH, GAME_HEIGHT));
    }
    
    for message in message_event.read() {
        
           
        if message.command == InputCommands::INVALID_INPUT {
            message_log.messages.push(MessageEntry {msg: "What???".to_string(), time: Timer::from_seconds(0.5, TimerMode::Once)});
        }
    
    }
    
    for message in message_log.messages.iter_mut().enumerate() {
        
        if !message.1.time.finished() {
            term.put_string(IVec2::new(GAMEBOARD_WIDTH+2, 0), (&message.1.msg).fg(FORE_COLOR).bg(Color::BLACK));
        } else {
            term.clear_box(IVec2::new(GAMEBOARD_WIDTH+2, 0), IVec2::new(14, 1));

        }
        
        message.1.time.tick(time.delta());
    }
}

pub fn handle_player_typed_input(input_string: String, mut event: EventWriter<InputCommandEvent>) {
    if input_string.to_ascii_lowercase().starts_with("inv") {
        event.send(InputCommandEvent{command: InputCommands::LIST_INVENTORY, data: String::new()});
    } else if input_string.to_ascii_lowercase().starts_with("drop"){
        let item_pos =input_string.find(" ");
        let mut item_string = String::new();
        if item_pos.is_some() {
            item_string = input_string.split_at(item_pos.unwrap()+1).1.to_string();
        } else {
            event.send(InputCommandEvent{command: InputCommands::INVALID_INPUT, data: String::new()});
            return
        }
        event.send(InputCommandEvent{command: InputCommands::DROP_ITEM, data: item_string});
    } else {
        event.send(InputCommandEvent{command: InputCommands::INVALID_INPUT, data: String::new()});
    }
}

