use std::io;
use std::io::{stdout, Write};
use std::collections::HashMap;
use rand::thread_rng;
use rand::seq::SliceRandom;


struct Board {
    marks: HashMap<i8,String>,
    state: [i8; 9],
    counter: u32,
}

impl Board {
    fn new() -> Self {
        let marks = HashMap::from([
            (0, String::from("X")),
            (1, String::from("O")),
        ]);
        Self { marks, state: [-1; 9], counter: 0 }
    } 

    fn render(&self) {
        let mut text: String = String::from("0|1|2
-----
3|4|5
-----
6|7|8");
        for (idx, x) in self.state.iter().enumerate() {
            if *x != -1 {
                text = text.replace(&idx.to_string(), &self.marks.get(x).unwrap());
            }
        } 
        println!("{}", text);
    }

    fn r#move(&mut self, idx: usize) -> bool {
        if self.state[idx] != -1 {
            println!("[Warning] すでに打たれています");
            return false;
        } 
        let player: i8 = (self.counter % 2) as i8;
        self.state[idx] = player;
        self.counter += 1;
        true
    }

    fn unmove(&mut self, idx: usize) {
        self.state[idx] = -1;
        self.counter -= 1;
    }

    fn is_win(&self, player: &i8) -> bool {
        let s = self.state;
        {
            (s[0] == s[1] && s[1] == s[2] && s[2] == *player) ||
            (s[3] == s[4] && s[4] == s[5] && s[5] == *player) ||
            (s[6] == s[7] && s[7] == s[8] && s[8] == *player) ||
            (s[0] == s[3] && s[3] == s[6] && s[6] == *player) ||
            (s[1] == s[4] && s[4] == s[7] && s[7] == *player) ||
            (s[2] == s[5] && s[5] == s[8] && s[8] == *player) ||
            (s[0] == s[4] && s[4] == s[8] && s[8] == *player) ||
            (s[2] == s[4] && s[4] == s[6] && s[6] == *player) 
        }
    }

    fn is_end(&self) -> bool {
        !self.state.iter().any(|e| *e==-1)
    }

    fn valid_moves(&self) -> Vec<i8> {
        let mut moves = Vec::new();
        for (idx, player) in self.state.iter().enumerate() {
            if *player == -1 {
                moves.push(idx as i8);
            }
        }
        moves
    }
}


enum Player {
    Random(RandomPlayer),
    Better(BetterPlayer),
    Best(BestPlayer),
    Human(HumanPlayer),
}

impl Player {
    fn play(&self, board: &mut Board) {
        match self {
            Player::Random(random) => random.play(board),
            Player::Better(better) => better.play(board),
            Player::Best(best) => best.play(board),
            Player::Human(human) => human.play(board),
        }    
    }
}


struct RandomPlayer;

impl RandomPlayer {
    fn play(&self, board: &mut Board) {
        let moves = board.valid_moves();
        let mut rng = thread_rng();
        let idx = moves.choose(&mut rng).unwrap();
        println!("ランダムプレイヤー: {}", idx);
        board.r#move(*idx as usize);    
    }
}


struct BetterPlayer {
    player: i8
}

impl BetterPlayer {
    fn new (player: i8) -> Self {
        Self { player }
    }


    fn play(&self, board: &mut Board) {
        let moves = board.valid_moves();

        for idx in &moves {
            board.r#move(*idx as usize);
            if board.is_win(&self.player) {
                println!("少し賢いプレイヤー: {}", idx);
                return
            }
            board.unmove(*idx as usize)
        }

        let mut rng = thread_rng();
        let idx = moves.choose(&mut rng).unwrap();
        println!("少し賢いプレイヤー: {}", idx);
        board.r#move(*idx as usize);    
    }
}


fn minimax(board: &mut Board, player: &i8) -> (f64, Option<i8>) {
    let maximize_player: i8 = 0;
    let minimize_player: i8 = 1;

    if board.is_win(&maximize_player) {
        return (1., None);
    }
    else if board.is_win(&minimize_player) {
        return (-1., None);
    }
    else if board.is_end() {
        return (0., None);
    }
    
    let opp: i8 = if *player == maximize_player { 1 } else { 0 };

    if *player == maximize_player {
        let mut max_score: f64 = -std::f64::INFINITY;
        let mut max_idx: Option<i8> = None;

        for idx in &board.valid_moves() {
            board.r#move(*idx as usize);
            let (score, _next_idx) = minimax(board, &opp);
            if max_score < score {
                max_score = score;
                max_idx = Some(*idx);
            }
            board.unmove(*idx as usize);
        }
        return (max_score, max_idx);
    }
    else {
        let mut min_score: f64 = std::f64::INFINITY;
        let mut min_idx: Option<i8> = None;

        for idx in &board.valid_moves() {
            board.r#move(*idx as usize);
            let (score, _next_idx) = minimax(board, &opp);
            if min_score > score {
                min_score = score;
                min_idx = Some(*idx);
            }
            board.unmove(*idx as usize);
        }
        return (min_score, min_idx);
    }
        
}


struct BestPlayer {
    player: i8
}

impl BestPlayer {
    fn new(player: i8) -> Self {
        Self { player }
    }

    fn play(&self, board: &mut Board) {
        let (_, idx) = minimax(board, &self.player);
        println!("最強のAI : {}", idx.unwrap());
        board.r#move(idx.unwrap() as usize);
    }
}


struct HumanPlayer;

impl HumanPlayer {
    fn play(&self, board: &mut Board) {
        loop {
            print!("0-8の数字を入力してください: ");
            stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("[Warning] 適切な値を入力してください");
            let input: i8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("[Warning] 適切な値を入力してください");
                    continue;
                }
            };
            if input < 0 || input > 8 {
                println!("[Warning] 0-8の数字を入力してください");
                continue;
            }
            if board.r#move(input as usize) {
                break;
            } else {
                continue;
            }
        }
    }
}


fn main() {
    let mut board = Board::new();
    let players: [Player; 2] = [Player::Best(BestPlayer::new(0)), Player::Human(HumanPlayer)];
    let mut player: i8 = 0;
    loop {
        let p = &players[player as usize];
        p.play(&mut board);
        board.render();

        if board.is_win(&player) {
            println!("{} の勝ち!", board.marks.get(&player).unwrap());
            break;
        }
        else if board.is_end() {
            println!("引き分け");
            break;
        }

        player = if player ==0 { 1 } else { 0 };
    }
}
