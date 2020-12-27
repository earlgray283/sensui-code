mod operation;
mod print;
mod sensui;

use print::Print;
use sensui::{get_enemy_action, AttackResult, EnemyAction, EnemyAttackResult, SensuiMap};

const INF: i32 = 1e9 as i32 + 7;
const MINF: i32 = -(1e9 as i32 + 7);

#[rustfmt::skip]
const MY_SENSUI_MAP: [&str; 5] = [
    "..#..", 
    ".....", 
    "#...#", 
    ".....", 
    "..#.."
];
const FIRST_ATTACK: (usize, usize) = (3, 2);

fn main() {
    let mut my_sensui = SensuiMap::new(MY_SENSUI_MAP.iter().map(|s| s.chars().collect()).collect());

    let mut table = vec![vec![-1; 5]; 5];
    let mut enemy_attacked_table = vec![vec![0; 5]; 5];

    let mut is_my_turn = true;
    let mut target = FIRST_ATTACK;
    let mut my_result = AttackResult::NONE;
    let mut enemy_result = EnemyAttackResult::NONE;

    loop {
        my_sensui.print_deco();
        table.print_deco();

        match is_my_turn {
            true => {
                match enemy_result {
                    // move
                    EnemyAttackResult::HIT(id) | EnemyAttackResult::RAGE(id) => {
                        // todo
                        dbg!(id);
                        let next = operation::mov(id, &my_sensui, &enemy_attacked_table);
                        my_sensui
                            .move_sensui(id, next.0, next.1)
                            .map_err(|e| println!("{}", e))
                            .unwrap();
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

                        my_result = res.unwrap();
                        match my_result {
                            AttackResult::HIT(_) => table[target.1][target.0] = INF,
                            AttackResult::DEAD(_) => table[target.1][target.0] = MINF,
                            AttackResult::NONE => table[target.1][target.0] = 0,
                            AttackResult::RAGE(_) => {
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

                        target = operation::base_search(&my_sensui, &table)
                            .unwrap_or(operation::base_probability(&table).unwrap_or((0, 1)));
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
                        println!("target: {}, {}", x, y);
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
                    EnemyAction::MOVE { d, n } => match my_result {
                        AttackResult::HIT(t) => {
                            let dxy_ = sensui::direction_to_dxy(d, n as i32);
                            let next = set_next(dxy_, t);

                            if !next.0.is_none() && !next.1.is_none() {
                                table[t.1][t.0] = -1;
                                table[next.1.unwrap()][next.0.unwrap()] = INF;
                            }

                            // hit したやつが移動してないことが確定したらそれはあまり意味がないのでなにもしない
                        }
                        AttackResult::RAGE(t) => {
                            // t を中心とした9つのセルを d 方向に n 移動
                            // 意味あるかな。。
                        }
                        _ => {
                            // dead と none だったら何もしない(わかんないやつを移動させてもどうしようもないので)
                        }
                    },
                }
            }
        }

        is_my_turn = !is_my_turn;
    }
}

fn set_next(dxy_: (i32, i32), t: (usize, usize)) -> (Option<usize>, Option<usize>) {
    let mut next = (Some(t.0), Some(t.1));
    if dxy_.0.is_negative() {
        next.0 = next.0.unwrap().checked_sub(dxy_.0.abs() as usize);
    } else {
        next.0 = if next.0.unwrap() + dxy_.0.abs() as usize >= 5 {
            None
        } else {
            Some(next.0.unwrap() + dxy_.0.abs() as usize)
        }
    }
    if dxy_.1.is_negative() {
        next.1 = next.1.unwrap().checked_sub(dxy_.1.abs() as usize);
    } else {
        next.1 = if next.1.unwrap() + dxy_.1.abs() as usize >= 5 {
            None
        } else {
            Some(next.1.unwrap() + dxy_.1.abs() as usize)
        }
    }

    next
}
