use bevy::math::ivec2;
use bevy::prelude::*;
use bevy_ascii_terminal::StringFormatter;
use bevy_ascii_terminal::Terminal;
use bevy_ascii_terminal::TileFormatter;

use crate::inventory::Inventory;
use crate::Player;

use crate::FORE_COLOR;
use crate::GAMEBOARD_WIDTH;
use crate::GAME_HEIGHT;

use super::common_components::*;
use super::rooms::*;

#[derive(Component)]
pub struct Item {
    pub name: String,
    pub visible: bool,
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub visual: Renderable,
    pub pos: Position,
}

pub fn render_items(inventory: Query<(Option<&Inventory>, &Room), With<CurrentRoom>>, items: Query<(&Item, &Position, &Renderable)>, mut terminal: Query<&mut Terminal>) {
    let mut term = terminal.get_single_mut().unwrap();

    let inv = &inventory.get_single().unwrap();
    if inv.0.is_some() {
        for curr_item in &inv.0.unwrap().inventory {
            let item_data = items.get(curr_item.item).unwrap();
            term.put_char(IVec2::new(item_data.1.x, item_data.1.y), item_data.2.glyph.fg(item_data.2.fg).bg(item_data.2.bg));
            
            //clear area here and draw more than one item
            term.clear_box(IVec2::new(GAMEBOARD_WIDTH+1, GAME_HEIGHT-10), IVec2::new(2, 5));
            term.put_char(IVec2::new(GAMEBOARD_WIDTH+1, GAME_HEIGHT-2), item_data.2.glyph.fg(item_data.2.fg).bg(item_data.2.bg));
            term.put_string([GAMEBOARD_WIDTH+3 , GAME_HEIGHT-2], item_data.0.name.clone().fg(FORE_COLOR));
        }
    }
}  

pub fn is_player_on_item(mut inventory: Query<(Option<&mut Inventory>, &Room), With<CurrentRoom>>, items: Query<(&Item, &Position, &Renderable)>, mut player: Query<(&Position, Option<&mut Inventory>), (Without<CurrentRoom>, With<Player>)>) {
    let mut inv = inventory.get_single_mut().unwrap(); 
    let mut player_data = player.get_single_mut().unwrap();

    if inv.0.is_some() {
        for (curr_item_idx, curr_item) in inv.0.iter_mut().enumerate() {
                if !curr_item.inventory.is_empty() {
                let item_data = items.get(curr_item.inventory[curr_item_idx].item).unwrap();

                if player_data.0.x == item_data.1.x && player_data.0.y == item_data.1.y {
                    let temp = curr_item.inventory.remove(curr_item_idx);
                        if player_data.1.is_some() {
                        player_data.1.as_mut().unwrap().inventory.push(temp)
                        }
                }
            }
        }
    }
}
