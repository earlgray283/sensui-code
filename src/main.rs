use sensui_code::sensui::SensuiMap;

mod sensui;

const MY_SENSUI_MAP: [&str; 5] = ["..#..", ".....", "#...#", ".....", "..#.."];



fn main() {
    let mut my_map: Vec<Vec<char>> = MY_SENSUI_MAP.iter().map(|s| s.chars().collect()).collect();
    let my_sensui = SensuiMap::new(my_map);
    let is_my_turn = true;
    
    
    
}
