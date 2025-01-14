// use crate::actions::Actions;
// use crate::loading::TextureAssets;
// use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{helpers::ldtk::{LdtkMapBundle, LdtkMap}, GameState};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), startup);
        // app.add_systems(OnEnter(GameState::Playing), spawn_player)
            // .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let handle: Handle<LdtkMap> = asset_server.load("map/Typical_TopDown_example.ldtk");
    let handle: Handle<LdtkMap> = asset_server.load("map/map.ldtk");

    commands.spawn(LdtkMapBundle {
        ldtk_map: handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}



// fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
//     commands
//         .spawn(SpriteBundle {
//             texture: textures.bevy.clone(),
//             transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
//             ..Default::default()
//         })
//         .insert(Player);
// }

// fn move_player(
//     time: Res<Time>,
//     actions: Res<Actions>,
//     mut player_query: Query<&mut Transform, With<Player>>,
// ) {
//     if actions.player_movement.is_none() {
//         return;
//     }
//     let speed = 150.;
//     let movement = Vec3::new(
//         actions.player_movement.unwrap().x * speed * time.delta_seconds(),
//         actions.player_movement.unwrap().y * speed * time.delta_seconds(),
//         0.,
//     );
//     for mut player_transform in &mut player_query {
//         player_transform.translation += movement;
//     }
// }
