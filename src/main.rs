const CELL_LENGTH: usize = 101;
const LOOP_AMOUNT: u32 = 50;

fn main() {
    const RULESET: u8 = 90;
    const ALIVE_CHAR: char = '█'; // full block
    const DEAD_CHAR: char = ' ';
    const WALL_CHAR: char = '|';

    let mut cells = [false; CELL_LENGTH]; 
    cells[CELL_LENGTH / 2] = true;

    for _ in 0..LOOP_AMOUNT {
        print_cells(&cells, &ALIVE_CHAR, &DEAD_CHAR, &WALL_CHAR);
        cells = calculate_new_cells(cells, &RULESET);
    }
}

fn print_cells(cells: &[bool; CELL_LENGTH], alive_char: &char, dead_char: &char, wall_char: &char) {
    print!("{wall_char}");
    for cell in cells {
        print!("{}", if *cell {
            alive_char
        } else {
            dead_char
        })
    }
    println!("{wall_char}");
}

fn calculate_new_cells(cells: [bool; CELL_LENGTH], ruleset: &u8) -> [bool; CELL_LENGTH]{
    let mut new_cells = cells.clone();
    for i in 1..CELL_LENGTH - 1 {
        // get left, source, and right cells
        let left   = cells[i - 1]; // weird order to prevent panic on negative
        let source = cells[i];
        let right  = cells[i + 1];

        // make binary number
        let num: u8 = (left as u8) * 4 + (source as u8) * 2 + (right as u8);

        // bit operations to extract bool from ruleset
        new_cells[i] = *ruleset & (1 << num) != 0;
    }
    new_cells
}