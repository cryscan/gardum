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

use super::{cli::Opts, AppState, Authority};
use crate::characters::heroes::OwnerPlayer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_server_player_from_opts)
            .add_system_set(
                SystemSet::on_enter(AppState::LobbyMenu).with_system(create_server_player),
            )
            .add_system_set(
                SystemSet::on_in_stack_update(AppState::InGame).with_system(update_player_hero),
            );
    }
}

fn create_server_player_from_opts(commands: Commands, opts: Res<Opts>) {
    if opts.subcommand.is_some() {
        create_server_player(commands);
    }
}

fn create_server_player(mut commands: Commands) {
    commands
        .spawn_bundle(PlayerBundle {
            nickname: Nickname("New player".to_string()),
            ..Default::default()
        })
        .insert(Authority);
}

fn update_player_hero(
    mut commands: Commands,
    hero_query: Query<(Entity, &OwnerPlayer), Added<OwnerPlayer>>,
    mut player_query: Query<&mut PlayerHero>,
) {
    for (hero, player) in hero_query.iter() {
        if let Ok(mut player_hero) = player_query.get_mut(player.0) {
            player_hero.0 = hero;
        } else {
            commands.entity(player.0).insert(PlayerHero(hero));
        }
    }
}

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    nickname: Nickname,
    kills: Kills,
    deaths: Deaths,
    damage: Damage,
    healing: Healing,
}

/// Stores player name
#[derive(Component, Default)]
pub struct Nickname(pub String);

/// Used to keep statistics of the number of kills
#[derive(Component, Default, Debug, PartialEq)]
pub struct Kills(pub u32);

/// Used to keep statistics of the number of deaths
#[derive(Component, Default, Debug, PartialEq)]
pub struct Deaths(pub u32);

/// Used to keep statistics of the damage done
#[derive(Component, Default, Debug, PartialEq)]
pub struct Damage(pub u32);

/// Used to keep statistics of the healing done
#[derive(Component, Default, Debug, PartialEq)]
pub struct Healing(pub u32);

/// Used to store player's hero entity
#[derive(Component)]
pub struct PlayerHero(pub Entity);
