const WINDOW_WIDTH:  usize = 101;
const WINDOW_HEIGHT: usize = 50;

fn main() {
    const RULESET: u8 = 90;
    const ALIVE_CHAR: char = '█'; // full block
    const DEAD_CHAR: char = ' ';
    const WALL_CHAR: char = '|';

    let mut cells = [false; WINDOW_WIDTH]; 
    cells[WINDOW_WIDTH / 2] = true;

    for _ in 0..WINDOW_HEIGHT {
        print_cells(&cells, &ALIVE_CHAR, &DEAD_CHAR, &WALL_CHAR);
        cells = calculate_new_cells(cells, &RULESET);
    }
}

fn print_cells(cells: &[bool; WINDOW_WIDTH], alive_char: &char, dead_char: &char, wall_char: &char) {
    print!("{wall_char}");
    for cell in cells {
        let charToPrint = if *cell { alive_char } else { dead_char };
        print!("{charToPrint}")
    }
    println!("{wall_char}");
}

fn calculate_new_cells(cells: [bool; WINDOW_WIDTH], ruleset: &u8) -> [bool; WINDOW_WIDTH]{
    let mut new_cells = cells.clone();
    for i in 0..WINDOW_WIDTH {
        // get left, source, and right cells
        let left   = cells[(i + WINDOW_WIDTH - 1) % WINDOW_WIDTH]; // weird order to prevent panic on negative
        let source = cells[i];
        let right  = cells[(i + WINDOW_WIDTH + 1) % WINDOW_WIDTH];

        // make binary number
        let num: u8 = ((left as u8) << 2) | ((source as u8) << 1) | (right as u8);

        // bit operations to extract bool from ruleset
        new_cells[i] = *ruleset & (1 << num) != 0;
    }
    new_cells
}