pub mod game_logic;
pub mod world;
pub mod www;

use std::sync::{Arc, RwLock};

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web::Data, App, HttpServer};
use log::{error, info};

use crate::{
    world::{World, WorldGenerationSettings},
    www::AppState,
};

fn main() -> Result<(), std::io::Error> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let config = load_config();

    let world = Arc::from(RwLock::new(World::from(config)));

    let app_state = Data::new(AppState {
        world: Arc::clone(&world),
    });

    let gl_handle = Arc::clone(&world);
    let game_logic_thread = std::thread::spawn(|| game_logic::main_event_loop(gl_handle));

    let web_server_thread = std::thread::spawn(move || {
        actix_web::rt::System::new().block_on(async move {
            let mut server = HttpServer::new(move || {
                App::new()
                    .wrap(
                        Cors::default()
                            .allow_any_header()
                            .allow_any_method()
                            .allow_any_origin(),
                    )
                    .service(www::health)
                    .service(www::get_room)
                    .service(www::get_world_manifest)
                    .service(Files::new("/", "./frontend/dist").index_file("index.html"))
                    .app_data(app_state.clone())
            });
            server = match server.bind(("127.0.0.1", 8080)) {
                Ok(s) => s,
                Err(e) => {
                    error!("Server error: {}", e);
                    return;
                }
            };

            server = server.disable_signals();

            info!("Web server started sucessfully");
            server.run().await.unwrap();
        })
    });

    web_server_thread.join().unwrap();
    game_logic_thread.join().unwrap();

    Ok(())
}

fn load_config() -> WorldGenerationSettings {
    let mut settings: WorldGenerationSettings = match std::fs::read_to_string("world_gen.toml") {
        Ok(text) => match toml::from_str(&text) {
            Ok(settings) => settings,
            Err(e) => {
                eprintln!("Error opening config: {}", e);
                eprintln!(
                    "Should follow following format:\n\n{}",
                    toml::to_string_pretty(&WorldGenerationSettings::default()).unwrap()
                );

                Default::default()
            }
        },
        Err(e) => {
            error!("Error importing config using default");
            eprintln!("Error opening config: {}", e);
            Default::default()
        }
    };

    if settings.seed == 0 {
        settings.seed = rand::random();
    }

    settings
}
