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
                    if col.len() < board.winning_condition {
                        continue;
                    }
                    let mut vert_counter = 0;
                    for i in col {
                        if i != player {
                            vert_counter = 0;
                            continue;
                        }
                        vert_counter += 1;
                        if vert_counter == board.winning_condition {
                            return true;
                        }
                    }
                }
                false
            }
            Rules::HorizontalWin => {
                let max_idx = board.data.len() - board.winning_condition;
                for (idx, col) in board.data.iter().enumerate() {
                    if col.len() == 0 {
                        continue;
                    }
                    if idx > max_idx {
                        break;
                    }
                    let mut skip_rows = false;
                    for (row_idx, &row) in col.iter().enumerate() {
                        if row != *player {
                            continue;
                        }
                        for idx2 in idx + 1..idx + board.winning_condition {
                            if board.data[idx2].len() <= row_idx {
                                skip_rows = true;
                                break;
                            }
                            if board.data[idx2][row_idx] != *player {
                                break;
                            }
                            if idx2 == idx + board.winning_condition - 1 {
                                return true;
                            }
                        }
                        if skip_rows {
                            break;
                        }
                    }
                }
                false
            }
        };
        false
    }
}

struct Board {
    rows: usize,
    winning_condition: usize,
    data: Vec<Vec<Field>>,
    rules: Vec<Rules>,
}

impl Board {
    fn new(cols: usize, rows: usize, winning_condition: usize) -> Board {
        Board {
            rows,
            winning_condition,
            data: vec![Vec::new(); cols],
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
        if self.data.len() <= col {
            return Err("No such column");
        }
        if self.data[col].len() >= self.rows {
            Err("The selected column is full")
        } else {
            self.data[col].push(player);
            Ok(player)
        }
    }
}

struct BoardVisual {
    board: Board,
}

impl BoardVisual {
    fn new(board: Board) -> BoardVisual {
        BoardVisual { board }
    }

    fn draw(&self) {
        let s = "-".repeat(self.board.data.len() * 2 + 1);
        println!("{}", s);
        for row in (0..self.board.rows).rev() {
            let mut idx = 0;
            for col in &self.board.data {
                if col.len() <= row {
                    print! {"| "};
                } else {
                    if col[row] == Field::Player1 {
                        print!("|o");
                    } else {
                        print!("|x");
                    }
                }
                if idx == self.board.data.len() - 1 {
                    print!("|");
                }
                idx += 1;
            }
            println!();
            println!("{}", s);
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

    for i in 0..10 {
        match board.put_stone_in_col(Field::Player1, i) {
            Err(e) => println!("Column {}: {}", i, e),
            Ok(_) => (),
        };
    }

    match board.is_player_winning(Field::Player1) {
        true => println!("Player 1 wins"),
        false => println!("No win"),
    };

    let board_v = BoardVisual::new(board);
    board_v.draw();
}
