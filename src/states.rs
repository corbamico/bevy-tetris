use crate::bricks::{Board, Dot};
use crate::consts::*;
use crate::tetrom::{DotInBoard, NewBrickEvent};
use bevy::prelude::*;
struct ScoreText;
struct LinesText;
struct LevelText;

//for game start, and game over
pub struct GameText;
pub struct ScoreRes(pub u32);
pub struct LinesRes(pub u32);
pub struct LevelRes(pub u32);
pub struct GameScorePlugin;
impl Plugin for GameScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_resource(ScoreRes(0))
            .add_resource(LinesRes(0))
            .add_resource(LevelRes(1))
            .add_resource(GameState(GameStage::Start))
            .add_system_to_stage(stage::UPDATE, score_change.system())
            .add_system_to_stage(stage::UPDATE, lines_change.system())
            .add_system_to_stage(stage::UPDATE, level_change.system())
            .add_system_to_stage(stage::POST_UPDATE, hanle_game_state.system())
            .add_system_to_stage(stage::POST_UPDATE, score_change.system())
            .add_system_to_stage(stage::POST_UPDATE, lines_change.system())
            .add_system_to_stage(stage::POST_UPDATE, level_change.system());
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("assets/digital7mono.ttf").unwrap();
    commands.spawn(UiCameraComponents::default());
    spwan_text(
        &mut commands,
        font_handle,
        "000000",
        TEXT_SCORE_X,
        TEXT_SCORE_Y,
        ScoreText,
    );
    spwan_text(
        &mut commands,
        font_handle,
        "0",
        TEXT_LINES_X,
        TEXT_LINES_Y,
        LinesText,
    );
    spwan_text(
        &mut commands,
        font_handle,
        "0",
        TEXT_LEVEL_X,
        TEXT_LEVEL_Y,
        LevelText,
    );
    spwan_text(
        &mut commands,
        font_handle,
        STRING_GAME_START,
        TEXT_GAME_X,
        TEXT_GAME_Y,
        GameText,
    );
}
fn score_change(score_res: ChangedRes<ScoreRes>, mut query: Query<(&ScoreText, &mut Text)>) {
    for (_, mut text) in &mut query.iter() {
        text.value = format!("{:06}", score_res.0 % 1000000);
    }
}
fn lines_change(lines_res: ChangedRes<LinesRes>, mut query: Query<(&LinesText, &mut Text)>) {
    for (_, mut text) in &mut query.iter() {
        text.value = format!("{:06}", lines_res.0 % 1000000);
    }
}
fn level_change(level_res: ChangedRes<LevelRes>, mut query: Query<(&LevelText, &mut Text)>) {
    for (_, mut text) in &mut query.iter() {
        text.value = format!("{}", level_res.0 % 100);
    }
}
fn spwan_text(
    commands: &mut Commands,
    font_handle: Handle<Font>,
    s: &str,
    x: f32,
    y: f32,
    component: impl Component,
) {
    commands
        .spawn(TextComponents {
            text: Text {
                value: s.to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 16.0,
                    color: Color::BLACK,
                },
            },
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(x),
                    top: Val::Px(y),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with(component);
}

pub enum GameStage {
    Start,
    Playing,
    GameOver,
}
pub struct GameState(pub GameStage);

fn hanle_game_state(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut score_res: ResMut<ScoreRes>,
    mut lines_res: ResMut<LinesRes>,
    mut level_res: ResMut<LevelRes>,
    mut board: ResMut<Board>,
    keyboard: Res<Input<KeyCode>>,
    mut event_sender: ResMut<Events<NewBrickEvent>>,
    mut query: Query<(&GameText, &mut Text)>,
    mut dots: Query<(Entity, &Dot, &DotInBoard)>,
) {
    match game_state.0 {
        GameStage::Start => {
            if keyboard.just_pressed(KeyCode::Space) {
                event_sender.send(NewBrickEvent);
                game_state.0 = GameStage::Playing;
                for (_, mut text) in &mut query.iter() {
                    text.value = STRING_GAME_PLAYING.to_string();
                }
            }
        }
        GameStage::Playing => {}
        GameStage::GameOver => {
            if keyboard.just_pressed(KeyCode::Space) {
                board.clear();
                for (entity, _, _) in &mut dots.iter() {
                    commands.despawn_recursive(entity);
                }
                score_res.0 = 0;
                lines_res.0 = 0;
                level_res.0 = 1;

                event_sender.send(NewBrickEvent);
                game_state.0 = GameStage::Playing;
                for (_, mut text) in &mut query.iter() {
                    text.value = STRING_GAME_PLAYING.to_string();
                }
            }
        }
    }
}
