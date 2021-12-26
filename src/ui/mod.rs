/*
 *  Copyright © 2021 Hennadii Chernyshchyk <genaloner@gmail.com>
 *
 *  This file is part of Gardum.
 *
 *  Gardum is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published by
 *  the Free Software Foundation; either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Gardum is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a get of the GNU Affero General Public License
 *  along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 */

mod back_button;
mod custom_game_menu;
mod ingame_menu;
mod main_menu;

use bevy::prelude::*;

use back_button::BackButtonPlugin;
use custom_game_menu::CustomGameMenuPlugin;
use ingame_menu::IngameMenuPlugin;
use main_menu::MainMenuPlugin;

pub const MENU_MARGIN: f32 = 20.0;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(MainMenuPlugin)
            .add_plugin(IngameMenuPlugin)
            .add_plugin(CustomGameMenuPlugin)
            .add_plugin(BackButtonPlugin);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameMenuState {
    MainMenu,
    CustomGameMenu,
    IngameMenu,
}
