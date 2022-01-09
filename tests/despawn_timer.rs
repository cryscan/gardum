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

use std::time::Duration;

use bevy::prelude::*;

use gardum::{
    characters::despawn_timer::{DespawnTimer, DespawnTimerPlugin},
    core::AppState,
};

#[test]
fn despawn_timer_ticks() {
    let mut app = setup_app();
    let dummy = app.world.spawn().insert(DespawnTimer::from_secs(1)).id();

    app.update();
    app.update();

    let despawn_timer = app.world.get::<DespawnTimer>(dummy).unwrap();
    assert!(
        despawn_timer.elapsed() > Duration::default(),
        "Despawn timer should tick"
    );
}

#[test]
fn despawn_timer_destroys() {
    let mut app = setup_app();
    app.world.spawn().insert(DespawnTimer::default()).id();

    app.update();

    assert_eq!(
        app.world.entities().len(),
        0,
        "Despawn timer should destroy its entity after the time expires"
    );
}

fn setup_app() -> App {
    let mut app = App::new();
    app.add_state(AppState::InGame)
        .add_plugins(MinimalPlugins)
        .add_plugin(DespawnTimerPlugin);
    app
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn despawn_timer_from_secs() {
        const SECONDS: u64 = 4;

        let cooldown = DespawnTimer::from_secs(SECONDS);
        assert_eq!(cooldown.duration(), Duration::from_secs(SECONDS));
        assert!(
            !cooldown.finished(),
            "Despawn timer should tick after creation"
        );
    }
}
