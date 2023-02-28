use bevy::prelude::{*, system_adapter::new};
use scrab::hex_grid::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init)
        .run()
}


fn init(mut c: Commands) {
    c.spawn(Camera2dBundle::default());

}
