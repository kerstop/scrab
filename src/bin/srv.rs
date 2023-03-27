use std::sync::{mpsc::channel, RwLock};

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web::Data, App, HttpServer};
use log::info;

use scrab::{world::World, www};

fn main() -> Result<(), std::io::Error> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let world = RwLock::new(World::new());

    let server_handle = Data::new(world);

    let (tx, rx) = channel::<Result<(), std::io::Error>>();

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
                    .app_data(server_handle.clone())
            });
            server = match server.bind(("127.0.0.1", 8080)) {
                Ok(s) => s,
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                    return;
                }
            };

            tx.send(Ok(())).unwrap();
            server.run().await.unwrap();
        })
    });

    //wait for the http server to start and
    //check for errors from server start.
    rx.recv().unwrap()?;
    info!("Web server started sucessfully");

    web_server_thread.join().unwrap();

    Ok(())
}
