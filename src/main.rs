use sensui_code::sensui::{get_enemy_action, SensuiMap};
use sensui_code::sensui::{AttackResult, Direction, EnemyAction};

mod sensui;

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

                let res = my_sensui.attack((target.1, target.0));
                if let Err(e) = &res {
                    eprintln!("{}", e);
                }

                let res = res.unwrap();
                match res {
                    AttackResult::HIT => table[target.1][target.0] = INF,
                    AttackResult::DEAD => {
                        table[target.1][target.0] = -1;
                    }
                    AttackResult::NONE => {
                        let range_y =
                            target.1.checked_sub(1).unwrap_or_default()..(target.1 + 1).min(5);
                        for i in range_y {
                            let range_x =
                                target.0.checked_sub(1).unwrap_or_default()..(target.0 + 1).min(5);
                            for j in range_x {
                                table[i][j] = 0;
                            }
                        }
                    }
                    AttackResult::RAGE => {
                        let range_y =
                            target.1.checked_sub(1).unwrap_or_default()..(target.1 + 2).min(5);
                        for i in range_y {
                            let range_x =
                                target.0.checked_sub(1).unwrap_or_default()..(target.0 + 2).min(5);
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

                target = set_target(&my_sensui, &table);
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

fn set_target(my_sensui: &SensuiMap, table: &Vec<Vec<i32>>) -> (usize, usize) {
    // 索敵を目的にしている
    // 関数化したほうが良さそう
    let mut list = Vec::new();
    for i in 0..5 {
        for j in 0..5 {
            if my_sensui.m[i][j] != '#' {
                list.push((j, i)); // (x, y) なので
            }
        }
    }

    let mut max = 0;
    let mut target = (5, 5);
    for t in list {
        let mut cnt = 0;
        for i in t.1.checked_sub(1).unwrap_or_default()..(t.1 + 2).min(5) {
            for j in t.0.checked_sub(1).unwrap_or_default()..(t.0 + 2).min(5) {
                if table[i][j] == -1 {
                    cnt += 1;
                }
            }
        }
        if cnt > max {
            println!("target: ({}, {}), cnt: {}", t.0, t.1, cnt);
            max = cnt;
            target = t;
        }
    }

    if target != (5, 5) {
        return target;
    }

    // max(table) の index を target にする
    let mut max = 0;
    for i in 0..5 {
        for j in 0..5 {
            if table[i][j] > max {
                target = (j, i);
                max = table[i][j];
            }
        }
    }

    target
}
