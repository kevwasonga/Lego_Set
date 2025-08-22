mod mobs;
mod world;

use mobs::{Mob, Boss, Member};
use world::World;

fn main() {
    let mut world = World::new(vec![
        Mob::new("Rusty Blades", Boss { name: "Don Rustoni".into(), power: 95 }, 500, "Rustville"),
        Mob::new("Bug Lords", Boss { name: "Carmen Codez".into(), power: 90 }, 300, "Bugtown"),
        Mob::new("Null Pointers", Boss { name: "Segfault Sam".into(), power: 85 }, 700, "Pointer City"),
    ]);

    // Example: recruiting before game loop
    world.mobs[0].recruit(Member { name: "Brick Brenda".into(), skill: "Negotiation".into() });

    for turn in 1..=5 {
        println!("--- Turn {} ---", turn);
        world.run_turn();
    }

    println!("\nFinal Standings:");
    for mob in world.mobs {
        println!("{:?}", mob);
    }
}
