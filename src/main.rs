use bevy::prelude::*;

fn main() {
    App::build()
        .add_system(greet.system())
        .run();
}


fn greet() {
    println!("hi system");
}