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
use bevy_inspector_egui::WorldInspectorParams;
use bevy_rapier3d::prelude::*;

use super::settings::{Settings, SettingsApplied};

pub(super) struct DeveloperPlugin;

impl Plugin for DeveloperPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::toggle_inspector_system)
            .add_system(Self::toggle_debug_collisions_system)
            .add_system(Self::update_inspector_setting_system);
    }
}

impl DeveloperPlugin {
    fn toggle_inspector_system(
        mut apply_events: EventReader<SettingsApplied>,
        settings: Res<Settings>,
        mut world_inspector: ResMut<WorldInspectorParams>,
    ) {
        if apply_events.iter().next().is_some() || settings.is_added() {
            world_inspector.enabled = settings.developer.world_inspector;
        }
    }

    fn toggle_debug_collisions_system(
        mut apply_events: EventReader<SettingsApplied>,
        settings: Res<Settings>,
        mut debug_render_ctx: ResMut<DebugRenderContext>,
    ) {
        if apply_events.iter().next().is_some() || settings.is_added() {
            debug_render_ctx.enabled = settings.developer.debug_collisions;
        }
    }

    /// Update the setting when closing the world inspector
    fn update_inspector_setting_system(
        mut settings: ResMut<Settings>,
        world_inspector: Res<WorldInspectorParams>,
    ) {
        if world_inspector.is_changed()
            && world_inspector.enabled != settings.developer.world_inspector
        {
            settings.developer.world_inspector = world_inspector.enabled;
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;

    use super::*;

    #[test]
    fn graphics_applies() {
        let mut app = App::new();
        app.add_plugin(TestDeveloperPlugin);

        app.update();

        let world_inspector = app.world.resource::<WorldInspectorParams>().enabled;
        let debug_collisions = app.world.resource::<DebugRenderContext>().enabled;
        let mut settings = app.world.resource_mut::<Settings>();
        assert_eq!(
            settings.developer.world_inspector, world_inspector,
            "World inspector setting should be loaded at startup"
        );
        assert_eq!(
            settings.developer.debug_collisions, debug_collisions,
            "Debug collisions setting should be loaded at startup"
        );

        settings.developer.world_inspector = !settings.developer.world_inspector;
        settings.developer.debug_collisions = !settings.developer.debug_collisions;

        let mut apply_events = app.world.resource_mut::<Events<SettingsApplied>>();
        apply_events.send(SettingsApplied);

        app.update();

        let world_inspector = app.world.resource::<WorldInspectorParams>().enabled;
        let debug_collisions = app.world.resource::<DebugRenderContext>().enabled;
        let settings = app.world.resource::<Settings>();
        assert_eq!(
            settings.developer.world_inspector, world_inspector,
            "World inspector setting should be loaded at startup"
        );
        assert_eq!(
            settings.developer.debug_collisions, debug_collisions,
            "Debug collisions setting should be loaded at startup"
        );
    }

    struct TestDeveloperPlugin;

    impl Plugin for TestDeveloperPlugin {
        fn build(&self, app: &mut App) {
            app.init_resource::<WorldInspectorParams>()
                .init_resource::<DebugRenderContext>()
                .init_resource::<Settings>()
                .add_event::<SettingsApplied>()
                .add_plugin(DeveloperPlugin);
        }
    }
}
