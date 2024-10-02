use std::io::{self, Write, Read, stdout};
use std::thread::sleep;
use std::time::{Instant, Duration};
use termion::{async_stdin, cursor, style};
use termion::raw::{IntoRawMode, RawTerminal};
use rand::{distributions::{Distribution, Standard}, Rng};


struct Game<R, W: Write> {
    /// The blocks on screen
    blocks:[[u8;10];20],

    /// The current piece under player control
    current_piece: FallingPiece,

    next_piece: FallingPiece,

    /// The current score
    score: u64,

    /// The rate of which pieces move down (controls difficulty)
    tickrate_ms: u64,

    game_over: bool,

    /// Input/Output
    stdout: W,
    stdin: R,
}

#[derive(Clone)]
struct FallingPiece {
    //piece_type:Tetramino,
    //rotation: usize,
    color: u8,
    piece_grid: Vec<Vec<bool>>, //[[bool;3];3],
    xpos: i16,
    ypos: i16
}

impl FallingPiece{
    fn new() -> FallingPiece{

        let ypos = 0;

        let tet: Tetramino = rand::random();

        match tet {
            Tetramino::I => {
                let mut piece_grid = vec![vec![false;4];4];

                piece_grid[1][0] = true;
                piece_grid[1][1] = true;
                piece_grid[1][2] = true;
                piece_grid[1][3] = true;


                FallingPiece{
                    xpos: 3,
                    ypos,
                    color: 36,
                    piece_grid  
                }
            },
            Tetramino::J => {
                let mut piece_grid = vec![vec![false;3];3];

                piece_grid[0][0] = true;
                piece_grid[1][0] = true;
                piece_grid[1][1] = true;
                piece_grid[1][2] = true;


                FallingPiece{
                    xpos: 3,
                    ypos,
                    color: 34,
                    piece_grid  
                }
            },
            Tetramino::L => {
                let mut piece_grid = vec![vec![false;3];3];

                piece_grid[0][2] = true;
                piece_grid[1][0] = true;
                piece_grid[1][1] = true;
                piece_grid[1][2] = true;


                FallingPiece{
                    xpos: 3,
                    ypos,
                    color: 33,
                    piece_grid  
                }

            },
            Tetramino::O => {
                let mut piece_grid = vec![vec![false;2];2];

                piece_grid[0][0] = true;
                piece_grid[1][0] = true;
                piece_grid[0][1] = true;
                piece_grid[1][1] = true;


                FallingPiece{
                    xpos: 4,
                    ypos,
                    color: 37,
                    piece_grid  
                }

            },
            Tetramino::S => {
                let mut piece_grid = vec![vec![false;3];3];

                piece_grid[0][1] = true;
                piece_grid[0][2] = true;
                piece_grid[1][0] = true;
                piece_grid[1][1] = true;


                FallingPiece{
                    xpos: 3,
                    ypos,
                    color: 32,
                    piece_grid  
                }

            },
            Tetramino::T => {
                let mut piece_grid = vec![vec![false;3];3];

                piece_grid[0][1] = true;
                piece_grid[1][0] = true;
                piece_grid[1][1] = true;
                piece_grid[1][2] = true;


                FallingPiece{
                    xpos: 3,
                    ypos,
                    color: 35,
                    piece_grid  
                }

            },
            Tetramino::Z => {
                let mut piece_grid = vec![vec![false;3];3];

                piece_grid[0][0] = true;
                piece_grid[0][1] = true;
                piece_grid[1][1] = true;
                piece_grid[1][2] = true;


                FallingPiece{
                    xpos: 3,
                    ypos,
                    color: 31,
                    piece_grid  
                }
            }
        }
    }
}

// the basic tetramino types
enum Tetramino{
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

// enable us to chose a tetramino at random
impl Distribution<Tetramino> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetramino {
        match rng.gen_range(0..=6) {
            0 => Tetramino::I,
            1 => Tetramino::J,
            2 => Tetramino::L,
            3 => Tetramino::O,
            4 => Tetramino::S,
            5 => Tetramino::T,
            _ => Tetramino::Z,
        }
    }
}


impl<R: Read, W: Write> Game<R, W> {

    /// Construct a new game
    fn new(stdin: R, stdout: W) -> Game<R, RawTerminal<W>> {
        Game {
            blocks: [[0;10];20],
            score: 0,
            current_piece: FallingPiece::new(),
            next_piece: FallingPiece::new(),
            tickrate_ms: 250,
            stdout: stdout.into_raw_mode().unwrap(),
            game_over: false,
            stdin,
        }
    
    }

    fn init(&mut self){
        write!(self.stdout, "{}{}", style::Reset, cursor::Hide).unwrap();
        self.stdout.flush().unwrap();
        println!("{}", "\n".repeat(21));
    }

    fn run(&mut self){

        let mut before = Instant::now();


        loop{
            let mut b = [0];
            if self.stdin.read(&mut b).is_ok() {
                match b[0] {
                    b'q' => break,
                    b'j' => self.move_piece(false),
                    b'l' => self.move_piece(true),
                    b'i' => self.attempt_rotatation(),
                    b'k' => while !self.step() {},
                    _ => {}
                };
                self.draw();
            }

            let now = Instant::now();
            let dt = (now.duration_since(before).subsec_millis()) as u64;

            if dt < self.tickrate_ms {
                sleep(Duration::from_millis(12));
                continue;
            }

            before = now;

            self.step();

            if self.game_over {
                break;
            }
            self.draw();
        }
    }

    fn draw(&mut self){
        draw_gameboard(20, 10, &self.blocks);
        draw_piece(&self.current_piece);
        draw_next_piece(20, 10, &self.next_piece);
    }

    fn move_piece(&mut self, direction: bool){
        match direction{
            false => {
                if !self.offset_position_collides(self.current_piece.clone(),-1, 0) {
                    self.current_piece.xpos -= 1;
                }
            }
            true => {
                if !self.offset_position_collides(self.current_piece.clone(),1, 0) {
                    self.current_piece.xpos += 1;
                }
            }
        }
    }

    fn attempt_rotatation(&mut self){
        let mut rotated_piece = self.current_piece.clone();
        rotated_piece.piece_grid = rotate_grid(rotated_piece.piece_grid);
        
        if !self.offset_position_collides(rotated_piece.clone(), 0, 0){
            self.current_piece = rotated_piece;
        }
    }

    /// Returns whether or not a block has been set
    fn step(&mut self) -> bool {
    
        if self.offset_position_collides(self.current_piece.clone(), 0, 1) {
            if self.current_piece.ypos == 0 {
                self.game_over = true;
                return true;
            }
            self.fuse_block();
            self.handle_completed_lines();
            self.current_piece = self.next_piece.clone();
            self.next_piece = FallingPiece::new();
            true
        } else {
            self.current_piece.ypos += 1;
            false
        }
    }
   
    /// Fuse the current falling block into the block grid
    fn fuse_block(&mut self){
        for (i, row) in self.current_piece.piece_grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                match col {
                    true => self.blocks[(self.current_piece.ypos as usize + i) as usize][(self.current_piece.xpos + j as i16) as usize] = self.current_piece.color,
                    _ => {},
                }
            }
        }
    }

    fn handle_completed_lines(&mut self){

        let mut completed_lines: Vec<usize> = Vec::new();

        for (i, row) in self.blocks.iter().enumerate() {
            if !row.contains(&0){
                completed_lines.push(i);
                self.score += 1;
            }
        }

        for row in completed_lines.iter(){
            for j in (1..=*row).rev() {
                self.blocks[j] = self.blocks[j-1].clone();
            }
        }

    }


    /// Check if an offset position collides
    fn offset_position_collides(&mut self, piece: FallingPiece, xoffset: i16, yoffset: usize) -> bool {
        let nextx:i16 = piece.xpos as i16 + xoffset;
        let nexty:usize = piece.ypos as usize + yoffset;

        for (i, row) in piece.piece_grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                match col {
                    true => if ((nexty + i) > 19) ||
                                (nextx + j as i16) < 0 ||
                                (nextx + j as i16) > 9 ||
                                (self.blocks[nexty + i][(nextx + j as i16) as usize] != 0)
                                {return true},
                    _ => {},
                }
            }
        }
        false
    }
}

impl<R, W: Write> Drop for Game<R, W> {
    fn drop(&mut self) {
        // When done, restore the defaults to avoid messing with the terminal.
        write!(self.stdout, "{}{}", style::Reset, cursor::Show).unwrap();
        self.stdout.write(b"\n\r").unwrap();
        println!("You scored: {} points\n\r", self.score);
    }
}

fn main() {
    // create space for us to draw the game
    let stdout = stdout();
    let mut game = Game::new(async_stdin(), stdout.lock());

    game.init();

    game.run();


}

fn draw_gameboard(height:usize, width:usize, block_array: &[[u8;10];20]) {

    // move cursor to top left
    print!("\u{001b}[{}A", height+1);
    print!("\u{001b}[{}D", 1000);
    io::stdout().flush().ok().expect("Could not flush stdout");

    // display top border
    println!("  ╔{}╗", "══".repeat(width));
    print!("\u{001b}[{}D", 1000);
    io::stdout().flush().ok().expect("Could not flush stdout");

    for (_, row) in block_array.iter().enumerate() {
        print!("  ║"); 

        for (_, col) in row.iter().enumerate() {
            // display a block if it exists
            match col {
                0 => print!("  "),
                _ => print!("\u{001b}[{};1m██", col),
            }
        }

        // reset color
        println!("\u{001b}[0m║{}", " ".repeat(20)); 
        print!("\u{001b}[{}D", 1000);
        io::stdout().flush().ok().expect("Could not flush stdout");
    }

    // display bottom border
    print!("  ╚{}╝", "══".repeat(width));
    print!("\u{001b}[{}D", 1000);
    io::stdout().flush().ok().expect("Could not flush stdout");
}

fn rotate_grid (piece_grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let grid_size = piece_grid.len();

    let mut rotated_grid = vec![vec![false;grid_size];grid_size];

    for (i, row) in piece_grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            // display a block if it exists
            match col {
                true => {
                    rotated_grid[j][grid_size-1-i] = true;
                },
                _ => {},
            }
        }
    }

    return rotated_grid;
}

fn draw_next_piece(height:usize, width:usize, piece: &FallingPiece) {
    // move to the top left
    print!("\u{001b}[{}A", height+1);
    print!("\u{001b}[{}D", 1000);
    print!("\u{001b}[{}C", 4+2*width);
    io::stdout().flush().ok().expect("Could not flush stdout");
    

    print!("    next",);

    print!("\u{001b}[{}B", 1); // move down
    print!("\u{001b}[{}D", 8); // move left 

    print!("  ┌{}┐", "──".repeat(piece.piece_grid.len()));

    print!("\u{001b}[{}B", 1); // move down
    print!("\u{001b}[{}D", piece.piece_grid.len()*2 + 4); // move left 


    for (_, row) in piece.piece_grid.iter().enumerate() {

        print!("  │");
        for (_, col) in row.iter().enumerate() {
            // display a block if it exists
            match col {
                true => print!("\u{001b}[{};1m██\u{001b}[0m", piece.color),
                _ => print!("  "),
            }
        }
        print!("│");
        print!("\u{001b}[{}B", 1); // move down
        print!("\u{001b}[{}D", piece.piece_grid.len()*2 + 4); // move left 
        io::stdout().flush().ok().expect("Could not flush stdout");
    }

    print!("  └{}┘", "──".repeat(piece.piece_grid.len()));
    print!("\u{001b}[{}B", 1); // move down
    print!("\u{001b}[{}D", piece.piece_grid.len()*2 + 8); // move left 

    print!("\u{001b}[0m"); 
    print!("\u{001b}[{}B", height-3); // move down
    print!("\u{001b}[{}D", 1000); // move left
    io::stdout().flush().ok().expect("Could not flush stdout");
}

fn draw_piece(piece: &FallingPiece) {
    print!("\u{001b}[{}D", 1000); // move left
    print!("\u{001b}[{}C", 2*piece.xpos+3); // move right
    print!("\u{001b}[{}A", 20-piece.ypos); // move up
    io::stdout().flush().ok().expect("Could not flush stdout");

    print!("\u{001b}[{};1m", piece.color);

    for (_, row) in piece.piece_grid.iter().enumerate() {

        for (_, col) in row.iter().enumerate() {
            // display a block if it exists
            match col {
                true => print!("██"),
                _ => print!("\u{001b}[2C"),
            }
        }
        print!("\u{001b}[{}B", 1); // move down
        print!("\u{001b}[{}D", piece.piece_grid.len()*2); // move left 
        io::stdout().flush().ok().expect("Could not flush stdout");
    }

    print!("\u{001b}[0m"); 
    print!("\u{001b}[{}B", 18-piece.ypos); // move down
    print!("\u{001b}[{}D", 1000); // move left
    io::stdout().flush().ok().expect("Could not flush stdout");
}

