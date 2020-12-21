use sensui::SensuiMap;
use crate::sensui;

/// 索敵を主として攻撃を決定
pub fn base_search(my_sensui: &SensuiMap, table: &Vec<Vec<i32>>) -> (usize, usize) {
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