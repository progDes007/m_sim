use bevy::app::App;
use bevy::DefaultPlugins;

pub struct BevyFront {}

impl BevyFront {
    pub fn new() -> Self {
        BevyFront {}
    }

    pub fn run(&self) 
    {
        App::new()
        .add_plugins(DefaultPlugins)
        .run();
    }
}
