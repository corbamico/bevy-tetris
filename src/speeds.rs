//delay = 725 * .85 ^ level + level (ms)
const TIMER_FALLING_SECS: f32 = 0.725;

pub fn get_speed(level: u32) -> f32 {
    vec![0.85; level as usize].iter().product::<f32>() * TIMER_FALLING_SECS
}

pub fn get_score(level: u32, erase_lines: u32) -> u32 {
    assert!(0 < erase_lines);
    assert!(erase_lines <= 4);
    vec![40, 100, 300, 1200][(erase_lines - 1) as usize] * level
}
pub fn get_level(total_lines: u32) -> u32 {
    1 + total_lines / 10
}
