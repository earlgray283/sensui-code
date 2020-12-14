use text_io::read;
pub enum AttackResult {
    HIT,
    RAGE,
    DEAD,
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
    m: Vec<Vec<char>>,
    e: Vec<Vec<char>>,
}

impl SensuiMap {
    pub fn new_rand() -> SensuiMap {
        let mut m = vec![vec!['.', '.', '.', '.', '.']; 5];
        for _i in 0..4 {
            let mut x = rand::random::<usize>() % 5;
            let mut y = rand::random::<usize>() % 5;
            while m[y][x] == '#' {
                x = rand::random::<usize>() % 5;
                y = rand::random::<usize>() % 5;
            }
            m[y][x] = '#';
        }

        SensuiMap {
            m,
            e: vec![vec!['*', '*', '*', '*', '*']; 5],
        }
    }

    pub fn new(m: Vec<Vec<char>>) -> SensuiMap {
        SensuiMap {
            m,
            e: vec![vec!['*', '*', '*', '*', '*']; 5],
        }
    }

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

        self.m[now.1][now.0] = '#';
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

    pub fn defence(&self) {
        /* todo */
    }

    
}

fn get_attack_result() -> AttackResult {
    loop {
        let s: String = read!();

        match &*s {
            "hit" => return AttackResult::HIT,
            "rage" => return AttackResult::RAGE,
            "dead" => return AttackResult::DEAD,
            _ => eprintln!("please input hit, rage or dead"),
        }
    }
}

fn get_enemy_action() -> EnemyAction {
    loop {
        let s: String = read!();
        let tokens: Vec<&str> = s.split(' ').collect();

        match tokens[0] {
            "1" => {
                if s.len() == 3 {
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
            "2" => {
                if s.len() == 3 {
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