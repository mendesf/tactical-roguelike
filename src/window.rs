use bevy::window::WindowResolution;
use bevy::{
    prelude::*,
    window::{close_on_esc, PresentMode, WindowTheme},
};

const WIDTH: f32 = 800.;

const HEIGHT: f32 = 600.;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(window_plugin())
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, (setup, make_visible))
        .add_systems(Update, close_on_esc);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Tactical Roguelike".into(),
            resolution: WindowResolution::new(WIDTH, HEIGHT),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            position: WindowPosition::Automatic,
            resizable: false,
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: false,
                ..Default::default()
            },
            // This will spawn an invisible window
            // The window will be made visible in the make_visible() system after 3 frames.
            // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
            visible: false,
            ..default()
        }),
        ..default()
    }
}

fn make_visible(mut window: Query<&mut Window>) {
    window.single_mut().visible = true;
}
