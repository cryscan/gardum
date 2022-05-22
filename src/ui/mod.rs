/*
 *  Copyright © 2021-2022 Hennadii Chernyshchyk <genaloner@gmail.com>
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
mod chat;
mod connection_dialog;
mod custom_game_menu;
mod error_message;
mod hero_selection;
mod hud;
mod ingame_menu;
mod main_menu;
mod perf_stats;
mod scoreboard;
mod settings_menu;
pub(super) mod ui_actions;
pub(super) mod ui_state;

use bevy::prelude::*;

use chat::ChatPlugin;
use connection_dialog::ConnectionDialogPlugin;
use custom_game_menu::CustomGameMenuPlugin;
use error_message::ErrorMessagePlugin;
use hero_selection::HeroSelectionPlugin;
use hud::HudPlugin;
use ingame_menu::InGameMenuPlugin;
use main_menu::MainMenuPlugin;
use perf_stats::PerfStatsPlugin;
use scoreboard::ScoreboardPlugin;
use settings_menu::SettingsMenuPlugin;
use ui_actions::UiActionsPlugin;
use ui_state::UiStatePlugin;

const UI_MARGIN: f32 = 20.0;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UiActionsPlugin)
            .add_plugin(UiStatePlugin)
            .add_plugin(ChatPlugin)
            .add_plugin(SettingsMenuPlugin)
            .add_plugin(HeroSelectionPlugin)
            .add_plugin(HudPlugin)
            .add_plugin(ScoreboardPlugin)
            .add_plugin(PerfStatsPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(CustomGameMenuPlugin)
            .add_plugin(ErrorMessagePlugin)
            .add_plugin(ConnectionDialogPlugin)
            .add_plugin(InGameMenuPlugin);
    }
}
