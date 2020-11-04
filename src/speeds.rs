const TIMER_FALLING_SECS: f32 = 0.725;

///tetris speeding
///delay = 725 * .85 ^ level + level (ms)
///use formula from dwhacks, http://gist.github.com/dwhacks/8644250
pub fn get_speed(level: u32) -> f32 {
    0.725 * (0.85 as f32).powi(level as i32) * TIMER_FALLING_SECS + level as f32 / 1000.0
}

///tetris scoring
///use as [Original Nintendo Scoring System]
///https://tetris.fandom.com/wiki/Scoring
pub fn get_score(level: u32, erase_lines: u32) -> u32 {
    assert!(0 < erase_lines);
    assert!(erase_lines <= 4);
    vec![40, 100, 300, 1200][(erase_lines - 1) as usize] * level
}

///level
///increase level every 10 lines.
pub fn get_level(total_lines: u32) -> u32 {
    let level = 1 + total_lines / 10;
    if level <= 99 {
        level
    } else {
        99
    }
}
