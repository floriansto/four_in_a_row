use std::clone::Clone;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Field {
    Player1,
    Player2,
}

enum Rules {
    VerticalWin,
    HorizontalWin,
}

impl Rules {
    fn evaluate(&self, board: &Board, player: &Field) -> bool {
        match self {
            Rules::VerticalWin => {
                for col in &board.data {
                    if col.data.len() < board.winning_condition {
                        continue;
                    }
                    let max_idx = col.data.len() - board.winning_condition;
                    for (idx, &i) in col.data.iter().enumerate() {
                        if idx > max_idx || i != *player {
                            continue;
                        }
                        let mut wins = true;
                        for j in &col.data[idx..board.winning_condition] {
                            wins &= j == player;
                            if !wins {
                                break;
                            }
                        }
                        if wins {
                            return wins;
                        }
                    }
                }
                false
            }
            Rules::HorizontalWin => false,
        };
        false
    }
}

#[derive(Debug, Clone)]
struct Column {
    max_height: usize,
    current_height: usize,
    data: Vec<Field>,
}

struct Board {
    winning_condition: usize,
    data: Vec<Column>,
    rules: Vec<Rules>,
}

impl Column {
    fn new(max_height: usize) -> Column {
        Column {
            max_height,
            current_height: 0,
            data: Vec::new(),
        }
    }
}

impl Board {
    fn new(cols: usize, rows: usize, winning_condition: usize) -> Board {
        Board {
            winning_condition,
            data: vec![Column::new(rows); cols],
            rules: Vec::new(),
        }
    }

    fn register_rule(&mut self, rule: Rules) {
        self.rules.push(rule)
    }

    fn is_player_winning(&self, player: Field) -> bool {
        for rule in &self.rules {
            match rule.evaluate(self, &player) {
                true => return true,
                false => (),
            };
        }
        false
    }

    fn put_stone_in_col(&mut self, player: Field, col: usize) -> Result<Field, &str> {
        let col: &mut Column = &mut self.data[col];
        if col.current_height >= col.max_height {
            Err("The selected column is full")
        } else {
            col.current_height += 1;
            col.data.push(player);
            Ok(player)
        }
    }
}

fn main() {
    let mut board = Board::new(7, 5, 4);

    board.register_rule(Rules::HorizontalWin);
    board.register_rule(Rules::VerticalWin);

    match board.put_stone_in_col(Field::Player2, 2) {
        Err(e) => println!("{}", e),
        Ok(_) => (),
    };

    for _ in 1..10 {
        match board.put_stone_in_col(Field::Player1, 2) {
            Err(e) => println!("{}", e),
            Ok(_) => (),
        };
    }

    match board.is_player_winning(Field::Player1) {
        true => println!("Player 1 wins"),
        false => println!("No win"),
    };
}
