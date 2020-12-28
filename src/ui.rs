use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::setup.system())
            .add_system(Self::update_frame_time.system());
    }
}

impl UiPlugin {
    fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
        commands.spawn(CameraUiBundle::default());
        commands
            .spawn(TextBundle {
                text: Text {
                    font: asset_server.load("fonts/Tuffy_Bold.ttf"),
                    style: TextStyle {
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(10.0),
                        left: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(FrameTime);
    }

    fn update_frame_time(
        diagnostics: Res<Diagnostics>,
        mut elements: Query<&mut Text, With<FrameTime>>,
    ) {
        for mut text in elements.iter_mut() {
            if let Some(frame_time) =
                diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
            {
                if let Some(frame_time) = frame_time.average() {
                    let frame_time_ms = frame_time * 1000.0;
                    text.value = format!("Frame Time: {:.0} ms", frame_time_ms);
                }
            }
        }
    }
}

struct FrameTime;
