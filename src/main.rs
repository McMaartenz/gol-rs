use std::{thread, fs::File, io::prelude::*, path::Path};
use std::time::{Duration, Instant};

const WIDTH: usize = 128;
const HEIGHT: usize = 48;

const FPS_CAP: f32 = 24.0;
const MILLIS: Duration = Duration::from_millis(((1.0 / FPS_CAP) * 1_000.0) as u64);

fn main() {
    let mut buffer = [[false;WIDTH];HEIGHT];

    load_from_file(&mut buffer);

    disp(&buffer);
    thread::sleep(Duration::from_millis(500));
    
    let mut start: Instant;
    loop {
        disp(&buffer);

        start = Instant::now();
        tick(&mut buffer);
        let frame_ms = start.elapsed();
        
        if frame_ms < MILLIS {
            let x = MILLIS - frame_ms - Duration::from_millis(1);
            
            println!("Frametime {} us, capped at {} FPS", frame_ms.as_micros(), FPS_CAP);
            thread::sleep(x);
            while Instant::now() < (start + x) {}
        }
    }
}

fn load_from_file(buffer: &mut [[bool;WIDTH];HEIGHT]) {
    let path = Path::new("initial_state.txt");
    let display = path.display();

	let mut file = match File::open(&path) {
		Err(why) => { println!("{}", why); return },
		Ok(file) => file,
	};

	let mut string = String::new();
	match file.read_to_string(&mut string) {
		Err(why) => { println!("Could not read {}: {}", display, why); return },
		Ok(_) => {},
	};

    let mut y = 0;
    let mut x = 0;
    string.lines().for_each(|line| {
        line.chars().for_each(|cell| { buffer[y][x] = cell == 'x'; x += 1; });
        y += 1;
        x = 0;
    });
}

fn disp(buffer: &[[bool;WIDTH];HEIGHT]) {
    let mut out = String::new();

    for ln in 0..HEIGHT {
        out.push_str(format!("{:2}", ln).as_str());
        buffer[ln].into_iter().for_each(|c| out.push(if c { '█' } else { ' ' }));
        out.push('\n');
    }

    println!("\x1B[1;1H{}", out);
}

fn tick(buffer: &mut [[bool;WIDTH];HEIGHT]) {
    let mut new_buff = [[false;WIDTH];HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let neighbor_count = count_neighbors(&buffer, y, x);
            new_buff[y][x] = neighbor_count == 3 || buffer[y][x] && neighbor_count == 2;
        }
    }

    /* Swap buffer */
    *buffer = new_buff;
}

#[inline(always)]
fn get_cell(buffer: &[[bool;WIDTH];HEIGHT], y: isize, x: isize) -> bool {
    if (x as usize) < WIDTH && (y as usize) < HEIGHT {
        return buffer[y as usize][x as usize];
    }
    
    false
}

fn count_neighbors(buffer: &[[bool;WIDTH];HEIGHT], y: usize, x: usize) -> u8 {
    const DELTA_Y: [isize;8] = [0,1,1,1,0,-1,-1,-1];
    const DELTA_X: [isize;8] = [1,1,0,-1,-1,-1,0,1];

    /* Bunch of get_cell calls */
    (0..8)
        .into_iter()
        .fold(0, |c,d| {
            return c + get_cell(buffer, y as isize + DELTA_Y[d], x as isize + DELTA_X[d]) as u8;
        })
}
