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

use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, Window},
    EguiContext,
};

use super::ui_state::UiState;
use crate::core::network::{client::ConnectionSettings, NetworkingState};

pub(super) struct ConnectionDialogPlugin;

impl Plugin for ConnectionDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::ConnectionDialog).with_system(connection_dialog_system),
        );
    }
}

fn connection_dialog_system(
    connection_setttings: Res<ConnectionSettings>,
    mut networking_state: ResMut<State<NetworkingState>>,
    mut egui: ResMut<EguiContext>,
    mut ui_state: ResMut<State<UiState>>,
) {
    if matches!(networking_state.current(), NetworkingState::Connected) {
        ui_state.set(UiState::LobbyMenu).unwrap();
        return;
    }

    Window::new("Connecting")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx_mut(), |ui| {
            ui.label(format!(
                "Connecting to {}:{}...",
                connection_setttings.ip, connection_setttings.port
            ));
            if ui.button("Cancel").clicked() {
                networking_state.set(NetworkingState::NoSocket).unwrap();
                ui_state.set(UiState::DirectConnectMenu).unwrap();
            }
        });
}
