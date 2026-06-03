mod matrix;
use matrix::Matrix;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut matrix = Matrix::new();

    matrix.run();
}

