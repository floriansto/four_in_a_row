use std::clone::Clone;
use std::io;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Field {
    Player1,
    Player2,
}

enum WinningCombinations {
    VerticalWin,
    HorizontalWin,
}

struct Rules {
    rules: Vec<WinningCombinations>,
    winning_condition: usize,
}

impl Rules {
    fn new(winning_condition: usize) -> Rules {
        Rules {
            rules: Vec::new(),
            winning_condition,
        }
    }

    fn evaluate(&self, board: &Board, player: &Field) -> bool {
        for rule in &self.rules {
            match rule {
                WinningCombinations::VerticalWin => {
                    match self.evaluate_vertical(board, player) {
                        true => return true,
                        false => (),
                    };
                }
                WinningCombinations::HorizontalWin => {
                    match self.evaluate_horizontal(board, player) {
                        true => return true,
                        false => (),
                    };
                }
            };
        }
        false
    }

    fn evaluate_vertical(&self, board: &Board, player: &Field) -> bool {
        for col in &board.data {
            if col.len() < self.winning_condition {
                continue;
            }
            let mut vert_counter = 0;
            for i in col {
                if i != player {
                    vert_counter = 0;
                    continue;
                }
                vert_counter += 1;
                if vert_counter == self.winning_condition {
                    return true;
                }
            }
        }
        false
    }

    fn evaluate_horizontal(&self, board: &Board, player: &Field) -> bool {
        let max_idx = board.data.len() - self.winning_condition;
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
                for idx2 in idx + 1..idx + self.winning_condition {
                    if board.data[idx2].len() <= row_idx {
                        skip_rows = true;
                        break;
                    }
                    if board.data[idx2][row_idx] != *player {
                        break;
                    }
                    if idx2 == idx + self.winning_condition - 1 {
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
}

struct Board {
    rows: usize,
    data: Vec<Vec<Field>>,
}

impl Board {
    fn new(cols: usize, rows: usize) -> Board {
        Board {
            rows,
            data: vec![Vec::new(); cols],
        }
    }

    fn draw(&self) {
        let s = "-".repeat(self.data.len() * 2 + 1);
        println!("{}", s);
        for row in (0..self.rows).rev() {
            let mut idx = 0;
            for col in &self.data {
                if col.len() <= row {
                    print! {"| "};
                } else {
                    if col[row] == Field::Player1 {
                        print!("|o");
                    } else {
                        print!("|x");
                    }
                }
                if idx == self.data.len() - 1 {
                    print!("|");
                }
                idx += 1;
            }
            println!();
            println!("{}", s);
        }
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

struct Game {
    rules: Rules,
    players: Vec<Field>,
}

impl Game {
    fn new(winning_condition: usize) -> Game {
        Game {
            rules: Rules::new(winning_condition),
            players: Vec::new(),
        }
    }

    fn register_rule(&mut self, rule: WinningCombinations) {
        self.rules.rules.push(rule)
    }

    fn is_player_winning(&self, board: &Board, player: Field) -> bool {
        match self.rules.evaluate(board, &player) {
            true => return true,
            false => return false,
        };
    }

    fn add_player(&mut self, player: Field) {
        self.players.push(player)
    }

    fn run(&mut self) {
        let mut board = Board::new(7, 5);

        self.register_rule(WinningCombinations::HorizontalWin);
        self.register_rule(WinningCombinations::VerticalWin);

        self.add_player(Field::Player1);
        self.add_player(Field::Player2);

        loop {
            for p in &self.players {
                let name;
                match p {
                    Field::Player1 => name = "Player 1".to_string(),
                    Field::Player2 => name = "Player 2".to_string(),
                }
                board.draw();
                println!("{} it's your turn!", name);
                let mut player_col = String::new();
                io::stdin()
                    .read_line(&mut player_col)
                    .expect("Failed to read line");

                let selection: usize = player_col.trim().parse().expect("Please type a number!");

                match board.put_stone_in_col(*p, selection) {
                    Err(e) => println!("Column {}: {}", selection, e),
                    Ok(_) => (),
                };

                match self.is_player_winning(&board, *p) {
                    true => {
                        println!("{} wins!", name);
                        board.draw();
                        return;
                    }
                    false => println!("Next player!"),
                };
            }
        }
    }
}

fn main() {
    let mut game = Game::new(4);
    game.run();
}
