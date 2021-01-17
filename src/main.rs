#![allow(dead_code)]
pub mod getch;
use getch::Getch;
fn clear() {
    println!("\x1b[2J\x1b[H"); // 清屏(clear)
}
enum GameState {
    Stopped,
    Playing,
    Pausing,
}
struct FrameWork {
    height: usize,
    width: usize,
    stringify: Vec<Vec<usize>>,
}
impl FrameWork {
    fn new(horizontal: usize, vertical: usize) -> Self {
        let mut temp = FrameWork {
            height: horizontal,
            width: vertical,
            stringify: vec![vec![0; vertical]; horizontal],
        };
        temp.draw(0, 0, vertical - 1, horizontal - 1);
        temp
    }
    fn rdraw(&mut self, up: usize, down: usize, left: usize, right: usize) {
        // Real Draw with real x, y crossover
        if up >= self.height
            || down >= self.height
            || left >= self.width
            || right >= self.width
            || left == right
            || up == down
        {
            eprintln!("\nThe Parameters You Had Given Is Incorrent!Function Draw will not work!\nTraceBack:\n\tleft:{} right:{} width_limit:{}\n\tup:{} down:{} height_limit:{}\n", left, right, self.width, up, down, self.height);
            return;
        }
        for i in (up + 1)..down {
            self.stringify[i][left] |= 3;
            self.stringify[i][right] |= 3;
        }
        for i in (left + 1)..right {
            self.stringify[up][i] |= 12;
            self.stringify[down][i] |= 12;
        }
        self.stringify[up][left] |= 6;
        self.stringify[up][right] |= 10;
        self.stringify[down][left] |= 5;
        self.stringify[down][right] |= 9;
    }
    fn draw(&mut self, x: usize, y: usize, width: usize, height: usize) {
        self.rdraw(
            self.height - 1 - y - height,
            self.height - 1 - y,
            x,
            x + width,
        );
    }
    fn get_string(&self) -> String {
        let vs = [
            ' ', '上', '下', '║', '左', '╚', '╔', '╠', '右', '╝', '╗', '╣', '═', '╩', '╦', '╬',
        ];
        let mut ret = String::new();
        for s in self.stringify.iter() {
            ret.push_str(&s.iter().map(|&x| vs[x]).collect::<String>());
            ret.push('\n');
        }
        ret
    }
    fn get_vec(&self) -> Vec<Vec<char>> {
        let vs = [
            '█', '上', '下', '║', '左', '╚', '╔', '╠', '右', '╝', '╗', '╣', '═', '╩', '╦', '╬',
        ];
        let mut ret = vec![];
        for s in self.stringify.iter() {
            ret.push(s.iter().map(|&x| vs[x]).collect::<Vec<char>>());
        }
        ret
    }
}
struct InterFace {
    interface: Vec<Vec<char>>,
    width: usize,
    height: usize,
}
impl InterFace {
    fn new() -> Self {
        let mut temp = InterFace {
            width: 70,
            height: 30,
            interface: vec![],
        };
        let mut frame = FrameWork::new(temp.height, temp.width);
        frame.rdraw(0, 29, 0, 40);
        frame.rdraw(4, 12, 48, 62);
        temp.interface = frame.get_vec();
        temp.write(50, 2, "Next Tetris");
        temp.write(43, 14, "Operations:");
        temp.write(43, 16, "s:");
        temp.write(47, 17, "pause the game");
        temp.write(43, 18, "q:");
        temp.write(47, 19, "quit the game");
        temp.write(43, 20, "↑↓←→:");
        temp.write(47, 21, "control the tertris");
        temp
    }
    fn write(&mut self, left: usize, top: usize, words: &'static str) {
        assert!(left + words.len() < self.width);
        for (i, ch) in words.chars().enumerate() {
            self.interface[top][i + left] = ch;
        }
    }
    fn show_frame(&self) {
        for line in &self.interface {
            for ch in line {
                print!("{}", ch);
            }
            print!("\n");
        }
    }
}
struct Game {
    state: GameState,
    interface: InterFace,
    blockes: [[bool; 10]; 50],
}
impl Game {
    fn new() -> Self {
        Game {
            state: GameState::Stopped,
            interface: InterFace::new(),
            blockes: [[false; 10]; 50],
        }
    }
    fn hook(&self) {
        let stdin = Getch::new();
        let mut chars = vec![];
        loop {
            let chu = stdin.getch();
            if chu == 'q' as u8 {
                println!("Are you sure to quit?y/n :");
                let ch: char = stdin.getch().into();
                if ch == 'y' {
                    println!("These are chars you had already put in:\n{:?}", chars);
                    return;
                } else {
                    println!("Back to game.");
                }
            } else {
                chars.push(chu);
                println!("{} ", chu);
            }
        }
    }
    fn start(&mut self) {
        clear();
        self.interface.show_frame();
    }
}
fn main() {
    let mut game = Game::new();
    game.start();
}
