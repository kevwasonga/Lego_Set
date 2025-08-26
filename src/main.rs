use serde::{Serialize, Deserialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use mobs::{Mob, Boss, Member};
use world::World;

mod mobs;
mod world;



fn main() {
    // Load initial data from mobs.json
    let data = fs::read_to_string("../data/mobs.json")
        .expect("Could not load mobs.json — maybe it's missing?");
    
    let mobs: Vec<Mob> = serde_json::from_str(&data)
        .expect("Failed to parse mobs.json");
    
    let mut game_world = World::new(mobs);
    
    // Create the output directory if it doesn't exist
    fs::create_dir_all("../output")
        .expect("Could not create output directory");
    
    // Run 5 turns of the simulation
    for turn in 1..=5 {
        println!("--- Turn {} ---", turn);
        game_world.run_turn();
        
        let json_output = serde_json::to_string_pretty(&game_world)
            .expect("Failed to serialize game state");
        
        let filename = format!("../output/turn_{}.json", turn);
        let mut file = File::create(&filename).expect("Could not create file");
        file.write_all(json_output.as_bytes()).expect("Could not write file");
        
        println!("Saved game state to {}", filename);
    }
}


//IDEA DOCS   https://docs.google.com/document/d/1L8VlcYtv-ybpx6lg8vtiyucx4cZLTyjHp0TJllYEWfU/edit?tab=t.0