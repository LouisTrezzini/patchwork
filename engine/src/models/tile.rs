use array2d::Array2D;

pub struct Tile {
    shape: Array2D<bool>,
    button_cost: u32,
    time_cost: u32,
    button_income: u32,
}

impl Tile {
    pub fn new(button_cost: u32, time_cost: u32, button_income: u32, shape_str: &str) -> Tile {
        let rows: Vec<Vec<bool>> = shape_str.split_whitespace().map(|x| x.chars().map(|c| {
            match c {
                'x' => true,
                'o' => false,
                _ => panic!()
            }
        }).collect()).collect();
        Tile {
            button_cost, time_cost, button_income, shape: Array2D::from_rows(&rows)
        }
    }

    pub fn display(&self) {
        println!("Button cost: {} / Time cost: {} / Button income: {}", self.button_cost, self.time_cost, self.button_income);
        for row in self.shape.rows_iter() {
            for cell in row {
                match cell {
                    true => print!("x"),
                    false => print!(" "),
                }
            }
            println!();
        }
    }
}