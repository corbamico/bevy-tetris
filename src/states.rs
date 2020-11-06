use crate::bricks::{Board, Dot};
use crate::consts::*;
use crate::screen::Materials;
use crate::speeds::{get_level, get_score, get_speed};
use crate::tetrom::{DotInBoard, NewBrickEvent};
use bevy::prelude::*;
struct ScoreText;
struct LinesText;
struct LevelText;

//for 'Game Start', and 'Game Over'
pub struct GameText;

pub enum GameState {
    Start,
    Playing,
    GameOver,
}
pub struct GameData {
    score: u32,
    lines: u32,
    level: u32,
    pub game_state: GameState,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            score: 0,
            lines: 0,
            level: 0,
            game_state: GameState::Start,
        }
    }
}
impl GameData {
    pub fn add_score(&mut self, score: u32) {
        self.score += score;
    }
    //return true, if level changed
    pub fn add_lines(&mut self, lines: u32) -> bool {
        let previous_level = self.level;
        self.score += get_score(self.level, lines);
        self.lines += lines;
        self.level = get_level(self.lines);
        previous_level != self.level
    }
    pub fn get_speed(&self) -> f32 {
        get_speed(self.level)
    }
}

pub struct GameScorePlugin;
impl Plugin for GameScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_resource(GameData::default())
            .add_system_to_stage(stage::UPDATE, handle_game_data.system())
            .add_system_to_stage(stage::POST_UPDATE, handle_game_state.system())
            .add_system_to_stage(stage::POST_UPDATE, handle_game_data.system());
    }
}

fn setup(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn(UiCameraComponents::default());
    spwan_text(
        &mut commands,
        materials.font.clone(),
        "000000",
        TEXT_SCORE_X,
        TEXT_SCORE_Y,
        ScoreText,
    );
    spwan_text(
        &mut commands,
        materials.font.clone(),
        "0",
        TEXT_LINES_X,
        TEXT_LINES_Y,
        LinesText,
    );
    spwan_text(
        &mut commands,
        materials.font.clone(),
        "00",
        TEXT_LEVEL_X,
        TEXT_LEVEL_Y,
        LevelText,
    );
    spwan_text(
        &mut commands,
        materials.font.clone(),
        STRING_GAME_START,
        TEXT_GAME_X,
        TEXT_GAME_Y,
        GameText,
    );
}

fn handle_game_data(
    game_data: ChangedRes<GameData>,
    mut score: Query<(&ScoreText, &mut Text)>,
    mut lines: Query<(&LinesText, &mut Text)>,
    mut level: Query<(&LevelText, &mut Text)>,
) {
    for (_, mut text) in &mut score.iter_mut() {
        text.value = format!("{:06}", game_data.score % 1000000);
    }
    for (_, mut text) in &mut lines.iter_mut() {
        text.value = format!("{:06}", game_data.lines % 1000000);
    }
    for (_, mut text) in &mut level.iter_mut() {
        text.value = format!("{:02}", game_data.level % 100);
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

fn handle_game_state(
    mut commands: Commands,
    //_game_data: ChangedRes<GameData>,
    mut game_data: ResMut<GameData>,
    mut board: ResMut<Board>,
    keyboard: Res<Input<KeyCode>>,
    mut event_sender: ResMut<Events<NewBrickEvent>>,
    mut query: Query<(&GameText, &mut Text)>,
    dots: Query<(Entity, &Dot, &DotInBoard)>,
) {
    match game_data.game_state {
        GameState::Start => {
            if keyboard.just_pressed(KeyCode::Space) {
                event_sender.send(NewBrickEvent);
                game_data.game_state = GameState::Playing;
                for (_, mut text) in &mut query.iter_mut() {
                    text.value = STRING_GAME_PLAYING.to_string();
                }
            }
        }
        GameState::Playing => {}
        GameState::GameOver => {
            if keyboard.just_pressed(KeyCode::Space) {
                board.clear();
                for (entity, _, _) in &mut dots.iter() {
                    commands.despawn_recursive(entity);
                }
                game_data.score = 0;
                game_data.lines = 0;
                game_data.level = 0;

                event_sender.send(NewBrickEvent);
                game_data.game_state = GameState::Playing;
                for (_, mut text) in &mut query.iter_mut() {
                    text.value = STRING_GAME_PLAYING.to_string();
                }
            }
        }
    }
}
