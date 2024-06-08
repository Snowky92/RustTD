use bevy::prelude::*;
use super::components::*;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::BufReader;

/**
 * Permet de charger la map depuis un fichier JSON
*/
pub fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let file_path = "map.json";
    let mut start = (0.0, 0.0);
    let mut end = (0.0, 0.0);
    match read_json_file(file_path) {
        Ok(map) => {
            for (i, inner_vec) in map.cases.iter().enumerate() {
                for (j, &value) in inner_vec.iter().enumerate() { 

                    // Si point de départ
                    if value == 0 {
                        start = ((j as f32 * 64.0 + 32.0), (i as f32 * 64.0 + 64.0));
                    }

                    // Si point de fin
                    if value == 1 {
                        end = ((j as f32 * 64.0 + 32.0), (i as f32 * 64.0 + 64.0));
                    }
                                       
                    let tile_path = match value {
                        0 => "sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile065.png",
                        1 => "sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile067.png",
                        2 => "sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile119.png",
                        3 => "sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile093.png",
                        _ => "sprites/kenney_tower-defense-top-down/PNG/Default size/towerDefense_tile093.png"
                    };

                    let transform = Transform::from_xyz(j as f32  * 64.0 + 32.0, i as f32 * 64.0 + 64.0, 0.0);

                    commands.spawn(SpriteBundle {
                        transform,
                        texture: asset_server.load(tile_path),
                        ..default()
                    });
                }
            };

            commands.spawn(Points {
                start,
                end
            });
        },
        Err(e) => println!("Error reading JSON file: {}", e)
    }
}
// petite fonction utilitaire pour charger un fichier JSON
fn read_json_file(file_path: &str) -> Result<Map> {
    let file = File::open(file_path).map_err(serde_json::Error::io)?;
    let reader = BufReader::new(file);
    let map = serde_json::from_reader(reader)?;
    Ok(map)
}