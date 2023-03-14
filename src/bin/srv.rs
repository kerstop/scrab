use std::sync::mpsc::channel;

use actix_files::Files;
use actix_web::{App, HttpServer};
use log::{error, info};

use scrab::www;

fn main() -> Result<(), std::io::Error> {

    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let (tx, rx) = channel::<Result<(), std::io::Error>>();

    let web_server_thread = std::thread::spawn(move || {
        actix_web::rt::System::new().block_on(async move {
            let mut server = HttpServer::new(|| {
                App::new()
                    .service(www::health)
                    .service(Files::new("/", "./frontend/dist").index_file("index.html"))
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
