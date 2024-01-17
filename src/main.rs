const CELL_LENGTH: usize = 30;
const LOOP_AMOUNT: u32 = 30;

fn main() {
    const RULESET: u8 = 0b01001101;
    const ALIVE_CHAR: char = '█'; // full block
    const DEAD_CHAR: char = ' ';
    const WALL_CHAR: char = '|';

    let mut cells = [false; CELL_LENGTH]; 
    cells[CELL_LENGTH / 2] = true;

    for i in 0..LOOP_AMOUNT {
        print_cells(&cells, &ALIVE_CHAR, &DEAD_CHAR, &WALL_CHAR);
        calculate_new_cells(&mut cells, &RULESET);
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

fn calculate_new_cells(cells: &mut [bool;CELL_LENGTH], ruleset: &u8) {
    for i in 0..CELL_LENGTH {
        // get left, source, and right cells with wraparound
        let left   = cells[(i + CELL_LENGTH - 1) % CELL_LENGTH]; // weird order to prevent panic on negative
        let source = cells[i];
        let right  = cells[(i + CELL_LENGTH + 1) % CELL_LENGTH];

        // make binary number
        let num = (left as usize) * 4 + (source as usize) * 2 + (right as usize);
    }
}