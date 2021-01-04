use crate::sensui;
use sensui::{Direction, SensuiMap};

/// 索敵を主として攻撃を決定
pub fn base_search(my_sensui: &SensuiMap, table: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
    let mut list = Vec::new();
    for i in 0..5 {
        for j in 0..5 {
            if my_sensui.m[i][j] != '#' && table[i][j] != 0 {
                list.push((j, i)); // (x, y) なので
            }
        }
    }

    let mut max = 0;
    let mut target = (5, 5);
    for t in list {
        if !my_sensui.is_attackable(t) {
            continue;
        }
        let mut cnt = 0;
        for i in t.1.checked_sub(1).unwrap_or_default()..(t.1 + 2).min(5) {
            for j in t.0.checked_sub(1).unwrap_or_default()..(t.0 + 2).min(5) {
                if table[i][j] == -1 {
                    cnt += 1;
                }
            }
        }

        //dbg!(cnt, target);
        if cnt > max {
            max = cnt;
            target = t;
        }
    }

    if target != (5, 5) {
        return Some(target);
    }

    None
}

/// 確率を主として攻撃決定.
pub fn base_probability(sensui: &SensuiMap, table: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
    let mut max = 0;
    let mut target = (5, 5);
    for i in 0..5 {
        for j in 0..5 {
            if sensui.m[i][j] == '#' {
                continue;
            }
            if !sensui.is_attackable((j, i)) {
                continue;
            }
            
            //dbg!((j, i), &table[i][j], max, target);
            if table[i][j] > max {
                target = (j, i);
                max = table[i][j];
            }
        }
    }

    if target != (5, 5) {
        return Some(target);
    }

    None
}

pub fn mov(id: usize, my_sensui: &SensuiMap, table: &Vec<Vec<i32>>) -> (Direction, usize) {
    let v = vec![
        Direction::EAST,
        Direction::WEST,
        Direction::NORTH,
        Direction::SOUTH,
    ];

    let mut ans: (Direction, usize) = (Direction::NORTH, 1);
    let mut _max = -1;
    for direction in &v {
        for i in 1..=2 {
            let dxy = sensui::direction_to_dxy(*direction, i);
            let next = super::set_next(dxy, my_sensui.sensuis[id].pos);
            if next.0.is_none() || next.1.is_none() {
                continue;
            }
            if my_sensui.m[next.1.unwrap()][next.0.unwrap()] == '#' {
                continue;
            }
            let next = (next.0.unwrap(), next.1.unwrap());

            if _max < table[next.1][next.0] {
                _max = table[next.1][next.0];
                ans = (*direction, i as usize);
            }
        }
    }

    ans
}
