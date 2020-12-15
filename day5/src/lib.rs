pub const ROW_MAX: u8 = 127;
pub const COLUMN_MAX: u8 = 7;

#[derive(Debug, Copy, Clone)]
pub struct BoardingPass {
    pub row: u8,
    pub column: u8,
    pub id: u32,
}

impl Default for BoardingPass {
    fn default() -> Self {
        BoardingPass {
            row: 0,
            column: 0,
            id: 0,
        }
    }
}

pub fn get_input() -> Vec<BoardingPass> {
    let file = include_str!("../input");
    let mut ret = Vec::new();

    for line in file.lines() {
        let raw_row = &line[0..7];
        let raw_column = &line[7..10];
        let mut pass = BoardingPass::default();

        let mut row_index = (0 as u8, ROW_MAX);
        for row_id in raw_row.chars() {
            match row_id {
                'F' => {
                    // Take the lower index
                    row_index.1 -= (row_index.1 - row_index.0) / 2;
                }
                'B' => {
                    // Take the upper index
                    row_index.0 += (row_index.1 - row_index.0) / 2;
                }
                _ => {
                    println!("Could not find match for: {:?}", row_id);
                }
            }
        }
        match raw_row.chars().last().unwrap() {
            'F' => pass.row = row_index.0 + 1,
            'B' => pass.row = row_index.1 + 1,
            _ => {}
        }

        let mut col_index = (0 as u8, COLUMN_MAX);
        for column_id in raw_column.chars() {
            match column_id {
                'L' => {
                    col_index.1 -= (col_index.1 - col_index.0) / 2;
                }
                'R' => {
                    col_index.0 += (col_index.1 - col_index.0) / 2;
                }
                _ => {
                    println!("Could not find match for: {:?}", column_id);
                }
            }
        }
        match raw_column.chars().last().unwrap() {
            'L' => pass.column = col_index.0 + 1,
            'R' => pass.column = col_index.1 + 1,
            _ => {}
        }

        pass.id = pass.row as u32 * 8 + pass.column as u32;

        ret.push(pass);
        println!("{:?}", pass);
    }

    ret
}
