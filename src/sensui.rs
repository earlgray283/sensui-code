use text_io::read;
pub enum AttackResult {
    HIT,
    RAGE,
    DEAD,
    NONE,
}

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
    pub hp_table: Vec<Vec<i32>>,
    e: Vec<Vec<char>>,
}

impl SensuiMap {
    pub fn new_rand() -> SensuiMap {
        let mut m = vec![vec!['.', '.', '.', '.', '.']; 5];
        for _i in 0..5 {
            let mut x = rand::random::<usize>() % 5;
            let mut y = rand::random::<usize>() % 5;
            while m[y][x] == '#' {
                x = rand::random::<usize>() % 5;
                y = rand::random::<usize>() % 5;
            }
            m[y][x] = '#';
        }

        let mut hp_table = vec![vec![-1, -1, -1, -1, -1]; 5];
        for i in 0..5 {
            for j in 0..5 {
                if m[i][j] == '#' {
                    hp_table[i][j] = 3;
                }
            }
        }

        SensuiMap {
            m,
            e: vec![vec!['*', '*', '*', '*', '*']; 5],
            hp_table
        }
    }

    pub fn new(m: Vec<Vec<char>>) -> SensuiMap {
        SensuiMap {
            m,
            e: vec![vec!['*', '*', '*', '*', '*']; 5],
            hp_table: vec![vec![3, 3, 3, 3, 3]; 5],
        }
    }

    // だめ
    pub fn move_sensui(&mut self, now: (usize, usize), next: (usize, usize)) -> bool {
        if now.1.checked_sub(1).is_none()
            || now.0.checked_sub(1).is_none()
            || next.0.checked_sub(1).is_none()
            || next.1.checked_sub(1).is_none()
            || now.1 >= 5
            || now.0 >= 5
            || next.1 >= 5
            || next.0 >= 5
        {
            return false;
        }

        if self.m[now.1][now.0] != '#' || self.m[next.1][next.0] == '#' {
            return false;
        }

        self.m[now.1][now.0] = '.';
        self.m[next.1][next.0] = '#';

        true
    }

    pub fn attack(&self, target: (usize, usize)) -> Result<AttackResult, String> {
        if self.m[target.1][target.0] == '#' {
            return Err(format!("({}, {}) == #", target.0, target.1));
        }

        println!("attack to ({}, {})", target.0, target.1);

        Ok(get_attack_result())
    }

    pub fn check_attack(&mut self, target: (usize, usize)) -> AttackResult {
        if self.m[target.1][target.0] == '#' {
            self.hp_table[target.1][target.0] -= 1;
            if self.hp_table[target.1][target.0] == 0 {
                return AttackResult::DEAD;
            }
            return AttackResult::HIT;
        }

        let range_y = target.1.checked_sub(1).unwrap_or_default()..(target.1+2).min(5);
        for i in range_y {
            let range_x = target.0.checked_sub(1).unwrap_or_default()..(target.0+2).min(5);
            for j in range_x {
                if self.m[i][j] == '#' {
                    return AttackResult::RAGE;
                }
            }
        }

        AttackResult::NONE
    }

    pub fn print(&self) {
        
    }
}


fn get_attack_result() -> AttackResult {
    loop {
        let s: String = read!();

        match &*s {
            "hit" => return AttackResult::HIT,
            "rage" => return AttackResult::RAGE,
            "dead" => return AttackResult::DEAD,
            "none" => return AttackResult::NONE,
            _ => eprintln!("please input hit, rage or dead"),
        }
    }
}

pub fn get_enemy_action() -> EnemyAction {
    loop {
        println!("input query > ");

        let s: String = read!("{}\n");
        let tokens: Vec<&str> = s.split(' ').collect();

        match tokens[0] {
            "1" => { // 1 x y
                if tokens.len() == 3 {
                    let x = tokens[1].parse::<usize>();
                    if let Err(e) = x {
                        eprintln!("{}", e);
                        continue;
                    }
                    let x = x.unwrap();

                    let y = tokens[2].parse::<usize>();
                    if let Err(e) = y {
                        eprintln!("{}", e);
                        continue;
                    }
                    let y = y.unwrap();

                    return EnemyAction::ATTACK { x, y };
                }
            }
            "2" => { // 2 d n
                if tokens.len() == 3 {
                    let d = tokens[1];
                    let n = tokens[2].parse::<usize>();
                    if let Err(e) = n {
                        eprintln!("{}", e);
                        continue;
                    }
                    let n = n.unwrap();

                    match d {
                        "north" => {
                            return EnemyAction::MOVE {
                                d: Direction::NORTH,
                                n: n,
                            }
                        }
                        "south" => {
                            return EnemyAction::MOVE {
                                d: Direction::SOUTH,
                                n: n,
                            }
                        }
                        "west" => {
                            return EnemyAction::MOVE {
                                d: Direction::WEST,
                                n: n,
                            }
                        }
                        "east" => {
                            return EnemyAction::MOVE {
                                d: Direction::EAST,
                                n: n,
                            }
                        }
                        _ => eprintln!("please input north, south, west or east"),
                    }
                }
            }
            _ => eprintln!("invalid query"),
        }
    }
}