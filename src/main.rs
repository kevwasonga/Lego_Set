use serde::{Serialize, Deserialize};

mod mobs;
mod world;
use std::fs;

use mobs::{Mob, Boss, Member};
use world::World;

fn main() {
    // use std::fs;

    // fn main() {
        let data = fs::read_to_string("data/mobs.json")
            .expect("Could not load mobs.json — maybe it's missing?");
            //match or if let:
    
        let mobs: Vec<Mob> = serde_json::from_str(&data)
            .expect("Failed to parse mobs.json");
    
        let mut game_world = World::new(mobs);
    
     
    // }
    

    // Example: recruiting before game loop
    // world.mobs[0].recruit(Member { name: "Brick Brenda".into(), skill: "Negotiation".into() });

    for turn in 1..=5 {
        println!("--- Turn {} ---", turn);
        game_world.run_turn();
    }

    println!("\nFinal Standings:");
    for mob in game_world.mobs {
        
        println!("{:#?}", mob);
    }
}
