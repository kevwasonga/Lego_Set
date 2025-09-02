use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};
use serde::{Serialize, Deserialize};
use std::{sync::{Arc, Mutex}, fs};
use mobs::Mob;
use world::World;

mod mobs;
mod world;

#[tokio::main]
async fn main() {
    // Load initial mobs.json
    let data = fs::read_to_string("../data/mobs.json")
        .expect("Could not load mobs.json");
    let mobs: Vec<Mob> = serde_json::from_str(&data).expect("Failed to parse mobs.json");
    
    // Shared state across requests
    let game_world = Arc::new(Mutex::new(World::new(mobs)));

    // Build router
    let app = Router::new()
        .route("/world/turn", post(run_turn))
        .route("/world/state", get(get_state))
        .with_state(game_world);

    println!("Server running at http://127.0.0.1:3000");        
let addr: std::net::SocketAddr = "127.0.0.1:3000".parse().unwrap();
let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
    .await
    .unwrap();

}

// POST /world/turn → advance 1 turn
async fn run_turn(State(state): State<Arc<Mutex<World>>>) -> Json<World> {
    let mut world = state.lock().unwrap();
    world.run_turn();
    world.sort_mobs_by_wealth();
    Json(world.to_owned())     //  if World: Clone

    
}

// GET /world/state → return current state
async fn get_state(State(state): State<Arc<Mutex<World>>>) -> Json<World> {
    let world = state.lock().unwrap();
    Json(world.to_owned())     //  if World: Clone
}
