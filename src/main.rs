use bevy::prelude::*;

mod credit_image;
mod menu;
mod network;
mod player;
mod procedural_functions;
mod states;
mod world;
mod save;

const TITLE: &str = "The Krusty Krabs";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

#[derive(Component)]
pub struct CharacterCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(states::StatePlugin)
        .add_plugin(credit_image::CreditImagePlugin)
        .add_plugin(menu::MenuPlugin)
        .insert_resource(WindowDescriptor {
            title: String::from(TITLE),
            width: WIN_W,
            height: WIN_H,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.6, 0.8)))
        .add_startup_system(|mut c: Commands| {
            c.spawn_bundle(Camera2dBundle::default())
                .insert(CharacterCamera);
        })
        .add_plugin(world::WorldPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(network::client::ClientPlugin)
        .add_plugin(network::server::ServerPlugin)
        .add_plugin(save::SaveLoadPlugin)
        .run();
}
