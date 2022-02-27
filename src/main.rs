#![deny(unsafe_code)]
use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine},
    event::{read, Event, KeyCode},
    execute,
    style::*,
    terminal::*,
    Result,
};
use lazy_static::lazy_static;
use std::io::stdout;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn clear() {
    let _t = execute!(stdout(), MoveTo(0, 0));
}

struct Game {
    state: GameState,
    interface: InterFace,
    blockes: [[u8; 10]; 20],
    curter: Tetris,
    nxtter: Tetris,
    scores: u128,
}

impl Game {
    fn new() -> Self {
        Game {
            state: GameState::Stopped,
            interface: InterFace::new(),
            blockes: [[0; 10]; 20],
            curter: Tetris::new(),
            nxtter: Tetris::new(),
            scores: 0,
        }
    }
    fn down(&mut self) {
        let mut bottom = false;
        for points in &TETRIS[self.curter.kind][2..] {
            let (xt, yt) = xt_yt(points, &self.curter);
            let xt = xt + 1;
            if !(xt < 0 || xt < 20 && self.blockes[xt as usize][yt as usize] == 0) {
                bottom = true;
                break;
            }
        }
        if bottom {
            for points in &TETRIS[self.curter.kind][2..] {
                let (xt, yt) = xt_yt(points, &self.curter);
                if xt < 0 || yt < 0 {
                    self.state = GameState::Stopped;
                    self.show_all();
                    return;
                }
                self.blockes[xt as usize][yt as usize] = self.curter.color;
            }
            let mut cleanpath = 0;
            for x in 0..20 {
                let mut filled = true;
                for y in 0..10 {
                    filled = filled & (self.blockes[x][y] != 0);
                }
                if filled {
                    cleanpath += 1;
                    for i in (1..=x).rev() {
                        self.blockes[i] = self.blockes[i - 1];
                    }
                    self.blockes[0] = [0; 10];
                }
            }
            if cleanpath > 0 {
                self.scores += 20u128.pow(cleanpath);
            }
            std::mem::swap(&mut self.curter, &mut self.nxtter);
            self.nxtter = Tetris::new();
        } else {
            self.curter.position[0] += 1;
        }
    }
    fn vertical(&mut self, moved: i32) {
        for points in &TETRIS[self.curter.kind][2..] {
            let (xt, yt) = xt_yt(points, &self.curter);
            let yt = yt + moved;
            if (yt < 0 || yt >= 10)
                || !(xt < 0 || xt < 20 && self.blockes[xt as usize][yt as usize] == 0)
            {
                return;
            }
        }
        self.curter.position[1] += moved;
    }
    fn turn(&mut self) {
        let _ = &self.curter.turn_right();
        for points in &TETRIS[self.curter.kind][2..] {
            let (xt, yt) = xt_yt(points, &self.curter);
            if (yt < 0 || yt >= 10)
                || !(xt < 0 || xt < 20 && self.blockes[xt as usize][yt as usize] == 0)
            {
                let _ = &self.curter.turn_left();
                return;
            }
        }
    }
    fn show_all(&self) {
        self.interface.show_frame(
            &self.curter,
            &self.nxtter,
            &self.blockes,
            &self.state,
            self.scores,
        );
    }
}

lazy_static! {
    static ref LOCK: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    static ref COLORTURN: Arc<Mutex<u8>> = Arc::new(Mutex::new(8));
    static ref THREAD_COUNT: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    static ref GAME: Arc<Mutex<Game>> = Arc::new(Mutex::new(Game {
        state: GameState::Stopped,
        interface: InterFace {
            width: 70,
            height: 30,
            interface: vec![],
        },
        blockes: [[0; 10]; 20],
        curter: Tetris {
            kind: 0,
            position: [-2, 5],
            direc: 0,
            color: 0,
        },
        nxtter: Tetris {
            kind: 0 as usize,
            position: [-2, 5],
            direc: 0 as usize,
            color: 0,
        },
        scores: 0,
    }));
}
const TETRIS: [[[i32; 2]; 6]; 7] = [
    // ? 数据格式：前两个为[x上限下限]，[y上限下限]，后面余下的为相对于旋转中心的坐标
    [[1, 0], [1, -1], [0, 0], [0, -1], [0, 1], [1, 0]], // T
    [[1, 0], [1, -1], [0, 0], [0, 1], [1, 0], [1, -1]], // S
    [[1, 0], [1, -1], [0, 0], [0, -1], [1, 0], [1, 1]], // Z
    [[1, -1], [0, -1], [0, 0], [-1, 0], [1, 0], [1, -1]], // J
    [[1, -1], [0, 1], [0, 0], [-1, 0], [1, 0], [1, 1]], // L
    [[1, -2], [0, 0], [0, 0], [-1, 0], [1, 0], [-2, 0]], // I
    [[1, 0], [1, 0], [0, 0], [1, 0], [0, 1], [1, 1]],   // O
];

#[derive(PartialEq, Clone)]
enum GameState {
    Stopped,
    Playing,
    Pausing,
}

fn xt_yt(points: &[i32; 2], t: &Tetris) -> (i32, i32) {
    (
        t.position[0] as i32
            + match t.direc {
                0 => points[0],
                1 => -points[1],
                2 => -points[0],
                _ => points[1],
            } as i32,
        t.position[1] as i32
            + match t.direc {
                0 => points[1],
                1 => points[0],
                2 => -points[1],
                _ => -points[0],
            } as i32,
    )
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
    fn get_vec(&self) -> Vec<Vec<char>> {
        let vs = [
            ' ', '上', '下', '║', '左', '╚', '╔', '╠', '右', '╝', '╗', '╣', '═', '╩', '╦', '╬',
        ];
        let mut ret = vec![];
        for s in self.stringify.iter() {
            ret.push(s.iter().map(|&x| vs[x]).collect::<Vec<char>>());
        }
        ret
    }
}

fn write(interface: &mut Vec<Vec<char>>, left: usize, top: usize, words: String) {
    assert!(left + words.len() < interface[0].len());
    for (i, ch) in words.chars().enumerate() {
        interface[top][i + left] = ch;
    }
}

fn write_styled(
    interface: &mut Vec<Vec<StyledContent<char>>>,
    left: usize,
    top: usize,
    words: String,
) {
    assert!(left + words.len() < interface[0].len());
    for (i, ch) in words.chars().enumerate() {
        interface[top][i + left] = style(ch);
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
        frame.rdraw(0, 29, 0, 39);
        frame.rdraw(6, 27, 9, 30);
        frame.rdraw(4, 12, 48, 61);
        temp.interface = frame.get_vec();
        write(&mut temp.interface, 49, 2, "Next Tetris".to_string());
        write(&mut temp.interface, 43, 14, "Operations:".to_string());
        write(&mut temp.interface, 43, 16, "space:".to_string());
        write(&mut temp.interface, 47, 17, "pause the game".to_string());
        write(&mut temp.interface, 43, 18, "q:".to_string());
        write(&mut temp.interface, 47, 19, "quit the game".to_string());
        write(&mut temp.interface, 43, 20, "↑↓←→:".to_string());
        write(
            &mut temp.interface,
            47,
            21,
            "control the tertris".to_string(),
        );
        temp
    }
    fn show_frame(
        &self,
        t: &Tetris,
        next: &Tetris,
        blockes: &[[u8; 10]; 20],
        state: &GameState,
        scores: u128,
    ) {
        clear();
        let mut interface = vec![];
        for lines in &self.interface {
            let mut line = vec![];
            for &ch in lines {
                line.push(style(ch))
            }
            interface.push(line);
        }
        for points in &TETRIS[t.kind][2..] {
            let (xt, yt) = xt_yt(points, t);
            if xt >= 0 && xt < 20 && yt >= 0 && yt < 10 {
                interface[xt as usize + 7][(yt as usize) * 2 + 10] =
                    style('█').with(Color::AnsiValue(t.color));
                interface[xt as usize + 7][(yt as usize) * 2 + 11] =
                    style('█').with(Color::AnsiValue(t.color));
            }
        }
        for points in &TETRIS[next.kind][2..] {
            interface[(points[0] + 8) as usize][(points[1] * 2 + 54) as usize] =
                style('█').with(Color::AnsiValue(next.color));
            interface[(points[0] + 8) as usize][(points[1] * 2 + 55) as usize] =
                style('█').with(Color::AnsiValue(next.color));
        }
        for x in 0..20 {
            for y in 0..10 {
                if blockes[x][y] >= 1 {
                    interface[x as usize + 7][(y as usize) * 2 + 10] =
                        style('█').with(Color::AnsiValue(blockes[x][y]));
                    interface[x as usize + 7][(y as usize) * 2 + 11] =
                        style('█').with(Color::AnsiValue(blockes[x][y]));
                }
            }
        }
        write_styled(
            &mut interface,
            17,
            5,
            match state {
                GameState::Playing => "Playing",
                GameState::Stopped => "You loose!",
                GameState::Pausing => "Pausing",
            }
            .to_string(),
        );
        write_styled(&mut interface, 16, 3, format!("Scores: {}", scores));
        for line in &interface {
            for &ch in line {
                print!("{}", ch);
            }
            let _ = execute!(stdout(), MoveToNextLine(1));
        }
    }
}

#[derive(Clone)]
struct Tetris {
    pub kind: usize,
    // 0-7分别表示 TSZJLIO型方块
    pub position: [i32; 2],
    // 表示了在游戏中的位置
    pub direc: usize,
    // 表示了方块在游戏中的指向，0为初始方向，1-3依次顺时针旋转90°
    pub color: u8, // 表示了方块在游戏中的颜色
}

impl Tetris {
    fn new() -> Self {
        Tetris {
            kind: (rand::random::<u8>() % 7) as usize,
            position: [-2, 5],
            direc: (rand::random::<u8>() % 4) as usize,
            color: {
                let mut color_turn_lock = COLORTURN.lock().unwrap();
                *color_turn_lock = (*color_turn_lock + 1) % 8;
                *color_turn_lock + 1
            },
        }
    }
    fn turn_right(&mut self) {
        self.direc += 1;
        self.direc %= 4;
    }
    fn turn_left(&mut self) {
        self.direc += 3;
        self.direc %= 4;
    }
}

fn main() -> Result<()> {
    if let Err(_) = enable_raw_mode() {
        println!("Your terminal does not support raw mode, please try another terminal or visit https://docs.rs/crossterm/0.23.0/crossterm/terminal/#raw-mode for more help.\
        Please Not Run This Program Directly in CLion/Intellij IDEA 's Run Tag!");
        exit(0);
    }
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    fn trans() {
        let mut begin_new_thread = false;
        {
            let mut game_lock = GAME.lock().unwrap();
            if game_lock.state == GameState::Playing {
                game_lock.state = GameState::Pausing;
                game_lock.show_all();
            } else {
                if game_lock.state == GameState::Stopped {
                    *game_lock = Game::new();
                }
                game_lock.state = GameState::Playing;
                game_lock.show_all();
                begin_new_thread = true
            }
        }
        if begin_new_thread {
            thread::spawn(move || {
                let _thread_holder = THREAD_COUNT.lock().unwrap();
                loop {
                    std::thread::sleep(Duration::from_millis(500));
                    let mut game_lock = GAME.lock().unwrap();
                    if game_lock.state != GameState::Playing {
                        return;
                    }
                    game_lock.down();
                    game_lock.show_all();
                }
            });
        }

    }
    let _ = execute!(stdout, Clear(ClearType::All));
    trans();
    loop {
        let event = read()?;
        if let Event::Key(_keyevent) = event {
            match _keyevent.code {
                KeyCode::Char(ch) => match ch {
                    'q' => break,
                    ' ' => trans(),
                    _ => (),
                },
                KeyCode::Up => {
                    let mut game_lock = GAME.lock().unwrap();
                    if game_lock.state == GameState::Playing {
                        game_lock.turn();
                        game_lock.show_all();
                    }
                }
                KeyCode::Down => {
                    let mut game_lock = GAME.lock().unwrap();
                    if game_lock.state == GameState::Playing {
                        game_lock.down();
                        game_lock.show_all();
                    }
                }
                KeyCode::Left => {
                    let mut game_lock = GAME.lock().unwrap();
                    if game_lock.state == GameState::Playing {
                        game_lock.vertical(-1);
                        game_lock.show_all();
                    }
                }
                KeyCode::Right => {
                    let mut game_lock = GAME.lock().unwrap();
                    if game_lock.state == GameState::Playing {
                        game_lock.vertical(1);
                        game_lock.show_all();
                    }
                }
                _ => {}
            }
        }
    }
    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
