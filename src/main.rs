mod sensui;
mod set_target;

use sensui::{get_enemy_action, SensuiMap};
use sensui::{AttackResult, Direction, EnemyAction};

const MY_SENSUI_MAP: [&str; 5] = ["..#..", ".....", "#...#", ".....", "..#.."];
const INF: i32 = 1e9 as i32 + 7;

pub trait Print<T> {
    fn print_all(&self);
}
impl<T: std::fmt::Display> Print<T> for Vec<Vec<T>> {
    fn print_all(&self) {
        for i in 0..5 {
            print!("[");
            for j in 0..5 {
                print!("{} ", self[i][j]);
            }
            println!("]");
        }
        println!();
    }
}

fn main() {
    let my_map: Vec<Vec<char>> = MY_SENSUI_MAP.iter().map(|s| s.chars().collect()).collect();
    let mut my_sensui = SensuiMap::new(my_map);
    let mut is_my_turn = true;

    let mut table = vec![vec![-1, -1, -1, -1, -1]; 5];

    let mut target = (2, 2);

    my_sensui.print_all();

    loop {
        table.print_all();

        match is_my_turn {
            true => {
                // attack or move
                let mut res: Result<AttackResult, String>;
                loop {
                    res = my_sensui.attack((target.1, target.0));
                    if let Err(e) = res {
                        println!("{}", e);
                        continue;
                    }
                    break;
                }

                let res = res.unwrap();
                match res {
                    AttackResult::HIT => table[target.1][target.0] = INF,
                    AttackResult::DEAD => {
                        table[target.1][target.0] = -1;
                    }
                    AttackResult::NONE => {
                        table[target.1][target.0] = 0;
                    }
                    AttackResult::RAGE => {
                        let range_y =
                            target.1.checked_sub(1).unwrap_or_default()..=(target.1 + 1).min(4);
                        for i in range_y {
                            let range_x =
                                target.0.checked_sub(1).unwrap_or_default()..=(target.0 + 1).min(4);
                            for j in range_x {
                                table[i][j] = if table[i][j] == -1 {
                                    1
                                } else {
                                    table[i][j] + 1
                                };
                            }
                        }
                        table[target.1][target.0] = -1;
                    }
                }

                target = set_target::base_search(&my_sensui, &table);
            }
            false => {
                // deffence
                let action = get_enemy_action();
                match action {
                    EnemyAction::ATTACK { x, y } => {
                        table[y][x] = -1;
                        match my_sensui.attack_response((x, y)) {
                            AttackResult::HIT => {
                                println!("hit");
                                my_sensui.hp_table[y][x] -= 1;
                            }
                            AttackResult::DEAD => {
                                println!("dead");
                                my_sensui.hp_table[y][x] = -1;
                                table[y][x] = -1;
                            }
                            AttackResult::NONE => println!("none"),
                            AttackResult::RAGE => println!("rage"),
                        }
                    }
                    EnemyAction::MOVE { d, n } => {}
                }
            }
        }

        is_my_turn = !is_my_turn;
    }
}
