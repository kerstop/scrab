use std::sync::mpsc::channel;

use actix_web::{App, HttpServer};
use log::info;

use scrab::www;

fn main() -> Result<(), std::io::Error>{
    let (tx, rx) = channel::<Result<(), std::io::Error>>();

    let web_server_thread = std::thread::spawn(move || {
        actix_web::rt::System::new().block_on(async move {
            match HttpServer::new(|| App::new().service(www::health)).bind(("127.0.0.1", 8080)) {
                Ok(s) => {
                    tx.send(Ok(()));
                    s.run().await;
                }
                Err(e) => {
                    tx.send(Err(e));
                }
            };
        })
    });

    //wait for the http server to start and
    //check for errors from server start.
    rx.recv().unwrap()?;
    info!("Web server started sucessfully");

    web_server_thread.join().unwrap();

    Ok(())
}
