mod sensui;
mod set_target;
mod print;

use print::Print;
use sensui::{get_enemy_action, SensuiMap, AttackResult, Direction, EnemyAction, EnemyAttackResult};

const INF: i32 = 1e9 as i32 + 7;

#[rustfmt::skip]
const MY_SENSUI_MAP: [&str; 5] = [
    "..#..", 
    ".....", 
    "#...#", 
    ".....", 
    "..#.."
];

fn main() {
    let mut my_sensui = SensuiMap::new(MY_SENSUI_MAP.iter().map(|s| s.chars().collect()).collect());
    
    let mut table = vec![vec![-1; 5]; 5];
    let mut enemy_attacked_table = vec![vec![0; 5]; 5];

    let mut is_my_turn = true;
    let mut target = (2, 2);
    let mut enemy_result = EnemyAttackResult::NONE;

    loop {
        my_sensui.print_deco();
        table.print_deco();

        match is_my_turn {
            true => {
                match enemy_result {
                    // move
                    EnemyAttackResult::HIT(id) | EnemyAttackResult::RAGE(id) => {
                        
                    }
                    // attack
                    _ => {
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
                            AttackResult::DEAD => table[target.1][target.0] = -1,
                            AttackResult::NONE => table[target.1][target.0] = 0,
                            AttackResult::RAGE => {
                                let range_y = target.1.checked_sub(1).unwrap_or_default()
                                    ..=(target.1 + 1).min(4);
                                for i in range_y {
                                    let range_x = target.0.checked_sub(1).unwrap_or_default()
                                        ..=(target.0 + 1).min(4);
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
                }
            }
            false => {
                // deffence
                let action = get_enemy_action();
                match action {
                    EnemyAction::ATTACK { x, y } => {
                        table[y][x] = -1;
                        enemy_attacked_table[y][x] += 1;
                        enemy_result = my_sensui.attack_response((x, y));
                        match enemy_result {
                            EnemyAttackResult::HIT(_) => {
                                println!("hit");
                            }
                            EnemyAttackResult::DEAD(_) => {
                                println!("dead");
                            }
                            EnemyAttackResult::RAGE(_) => {
                                println!("rage");
                            }
                            EnemyAttackResult::NONE => {
                                println!("none");
                            }
                        }
                    }
                    EnemyAction::MOVE { d, n } => {}
                }
            }
        }

        is_my_turn = !is_my_turn;
    }
}
