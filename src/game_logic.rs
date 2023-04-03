use std::{sync::{Arc, RwLock}, time::Duration};

use log::info;

use scrab_types::World;



pub fn main_event_loop(world: Arc<RwLock<World>>) {
    
    loop {
        info!("Starting tick {}", world.read().unwrap().current_tick);
        std::thread::sleep(Duration::from_secs(2));
        
        info!("Finishing tick {}", world.read().unwrap().current_tick);
        world.write().unwrap().current_tick += 1;
    }
}