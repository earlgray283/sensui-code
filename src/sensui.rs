// 基本的に授業の潜水艦ゲームの仕様に準じています。
// 潜水艦ゲーム内での最低限の操作しか実装していません(最低限で200行ってそれマジ？)

use std::collections::HashMap;
use text_io::read;

// 自分の攻撃に対する相手の反応() 内は潜水艦の id
pub enum AttackResult {
    HIT((usize, usize)),
    RAGE((usize, usize)),
    DEAD((usize, usize)),
    NONE,
}

// enemy の攻撃に対する反応
pub enum EnemyAttackResult {
    HIT(usize),
    RAGE(Vec<usize>),
    DEAD(usize),
    NONE,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

pub enum EnemyAction {
    MOVE { d: Direction, n: usize },
    ATTACK { x: usize, y: usize },
}

pub struct SensuiMap {
    pub m: Vec<Vec<char>>,
    /// (usize, usize) = (x, y)
    pub id_map: HashMap<(usize, usize), usize>,
    pub sensuis: Vec<SensuiData>,
}

pub struct SensuiData {
    pub id: usize,
    pub hp: usize,
    pub pos: (usize, usize),
    pub status: AttackResult,
}

impl SensuiMap {
    pub fn new_rand() -> SensuiMap {
        let mut v = vec![vec!['.'; 5]; 5];
        for _ in 0..4 {
            loop {
                let x = rand::random::<usize>() % 5;
                let y = rand::random::<usize>() % 5;
                if v[y][x] != '#' {
                    v[y][x] = '#';
                    break;
                }
            }
        }

        SensuiMap::new(v)
    }

    pub fn new(m: Vec<Vec<char>>) -> SensuiMap {
        let mut pos_list = Vec::new();
        for i in 0..5 {
            for j in 0..5 {
                if m[i][j] == '#' {
                    pos_list.push((j, i));
                }
            }
        }

        let mut id_map: HashMap<(usize, usize), usize> = HashMap::new();
        let sensuis = (0..4)
            .map(|i| SensuiData {
                id: i,
                hp: 3,
                pos: pos_list[i],
                status: AttackResult::NONE,
            })
            .collect::<Vec<SensuiData>>();
        for sensui in &sensuis {
            id_map.insert(sensui.pos, sensui.id);
        }

        SensuiMap { m, sensuis, id_map }
    }

    pub fn move_sensui(&mut self, id: usize, direction: Direction, n: usize) -> Result<(), String> {
        let n = n as i32;
        let dxy = direction_to_dxy(direction, n);

        eprintln!(
            "【Action】move {} squares {}!",
            n,
            match direction {
                Direction::EAST => "east",
                Direction::NORTH => "north",
                Direction::SOUTH => "south",
                Direction::WEST => "west",
            }
        );

        self.m[self.sensuis[id].pos.1][self.sensuis[id].pos.0] = '.';

        if self.sensuis[id].hp == 0 {
            return Err("err".to_string())
        }

        let id_ = self.id_map[&self.sensuis[id].pos];
        self.id_map
            .remove(&self.sensuis[id].pos)
            .expect("【Error】map remove error");

        if dxy.0.is_negative() {
            self.sensuis[id].pos.0 -= dxy.0.abs() as usize;
        } else {
            self.sensuis[id].pos.0 += dxy.0.abs() as usize;
        }
        if dxy.1.is_negative() {
            self.sensuis[id].pos.1 -= dxy.1.abs() as usize;
        } else {
            self.sensuis[id].pos.1 += dxy.1.abs() as usize;
        }

        self.m[self.sensuis[id].pos.1][self.sensuis[id].pos.0] = '#';
        self.id_map.insert(self.sensuis[id].pos, id_);

        // todo

        Ok(())
    }

    pub fn is_attackable(&self, pos: (usize, usize)) -> bool {
        let atkble = self.gen_attackable();
        atkble[pos.1][pos.0]
    }

    pub fn gen_attackable(&self) -> Vec<Vec<bool>> {
        let mut attackable = vec![vec![false; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                if self.m[i][j] == '#' {
                    for ii in i.checked_sub(1).unwrap_or_default()..=(i + 1).min(4) {
                        for jj in j.checked_sub(1).unwrap_or_default()..=(j + 1).min(4) {
                            if ii == i && jj == j {
                                continue;
                            }
                            attackable[ii][jj] = true;
                        }
                    }
                }
            }
        }

        attackable
    }

    pub fn attack(&self, target: (usize, usize)) -> Result<AttackResult, String> {
        if self.m[target.1][target.0] == '#' {
            return Err(format!(
                "【Error】There is a submarine at ({}, {}) in your map",
                target.0, target.1
            ));
        }

        eprintln!(
            "【Action】attack to ({}, {})!",
            (target.1 as u8 + 'A' as u8) as char,
            target.0 + 1
        );

        Ok(get_attack_response(target))
    }

    pub fn attack_response(&mut self, target: (usize, usize)) -> EnemyAttackResult {
        if self.m[target.1][target.0] == '#' {
            let id = self.id_map[&target];
            self.sensuis[id].hp -= 1;
            if self.sensuis[id].hp == 0 {
                self.sensuis[id].status = AttackResult::DEAD(target);
                self.id_map.remove(&target);
                self.m[target.1][target.0] = '.';
                eprintln!("【Attack Response】Dead");
                return EnemyAttackResult::DEAD(id);
            }
            eprintln!("【Attack Response】Hit");
            return EnemyAttackResult::HIT(self.id_map[&target]);
        }

        let mut v = Vec::new();
        for i in target.1.checked_sub(1).unwrap_or_default()..=(target.1 + 1).min(4) {
            for j in target.0.checked_sub(1).unwrap_or_default()..=(target.0 + 1).min(4) {
                if self.m[i][j] == '#' {
                    v.push(self.id_map[&(j, i)]);
                }
            }
        }
        if !v.is_empty() {
            eprintln!("【Attack Response】Rage");
            return EnemyAttackResult::RAGE(v);
        }

        eprintln!("【Attack Response】None");
        EnemyAttackResult::NONE
    }

    // print decolated sensui-map
    pub fn print_deco(&self) {
        let mut v = Vec::new();

        let mut max = 0;
        for i in 0..5 {
            let mut buf = String::new();
            buf.push_str("| ");
            for j in 0..5 {
                buf.push_str(&format!("{} ", self.m[i][j]));
            }
            buf.push_str("|\n");
            max = max.max(buf.len() - 3);
            v.push(buf);
        }

        eprint!("+");
        for _ in 0..max {
            eprint!("-");
        }
        eprintln!("+");
        for s in &v {
            eprint!("{}", s);
        }
        eprint!("+");
        for _ in 0..max {
            eprint!("-");
        }
        eprintln!("+");
    }
}

fn get_attack_response(target: (usize, usize)) -> AttackResult {
    loop {
        eprintln!("【Prompt】input attack result: (hit / rage / dead / none) > ");
        let s: String = read!();
        println!("{}", &s);

        match &*s {
            "hit" => return AttackResult::HIT(target),
            "rage" => return AttackResult::RAGE(target),
            "dead" => return AttackResult::DEAD(target),
            "none" => return AttackResult::NONE,
            _ => eprintln!("【Error】invalid result"),
        }
    }
}

pub fn get_enemy_action() -> EnemyAction {
    loop {
        eprintln!("【Prompt】input query: (1 x y / 2 d n) > ");

        let s: String = read!("{}\n");
        println!("{}", &s);
        let tokens: Vec<&str> = s.split(' ').collect();

        match tokens[0] {
            "1" => {
                // 1 x y
                if tokens.len() == 3 {
                    let y = tokens[1].parse::<char>();
                    if let Err(e) = y {
                        eprintln!("{}", e);
                        continue;
                    }
                    let y = y.unwrap() as usize - 'A' as usize;

                    let x = tokens[2].parse::<usize>();
                    if let Err(e) = x {
                        eprintln!("{}", e);
                        continue;
                    }
                    let x = x.unwrap() - 1;

                    return EnemyAction::ATTACK { x, y };
                }
            }
            "2" => {
                // 2 d n
                if tokens.len() == 3 {
                    let d = tokens[1];
                    let n = tokens[2].parse::<usize>();
                    if let Err(e) = n {
                        eprintln!("{}", e);
                        continue;
                    }
                    let n = n.unwrap();
                    if d != "north" && d != "south" && d != "west" && d != "east" {
                        eprintln!("【Error】invalid direction");
                        continue;
                    }

                    return EnemyAction::MOVE {
                        d: match d {
                            "north" => Direction::NORTH,
                            "south" => Direction::SOUTH,
                            "west" => Direction::WEST,
                            "east" => Direction::EAST,
                            _ => Direction::NORTH,
                        },
                        n,
                    };
                }
            }
            _ => eprintln!("【Error】invalid query"),
        }
    }
}

pub fn direction_to_dxy(direction: Direction, n: i32) -> (i32, i32) {
    match direction {
        Direction::EAST => (n, 0),
        Direction::NORTH => (0, -n),
        Direction::SOUTH => (0, n),
        Direction::WEST => (-n, 0),
    }
}
