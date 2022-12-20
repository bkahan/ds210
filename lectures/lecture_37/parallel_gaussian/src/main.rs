use chrono::Local;
use clap::Parser;
//use rand::rngs::StdRng;
use rand::Rng;
//use rand::SeedableRng;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::ops::Sub;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Matrix size
    #[arg(short, long, default_value_t = 64)]
    size: usize,

    /// Number of threads
    #[arg(short, long, default_value_t = 1)]
    threads: usize,

    /// Display the matrix
    #[arg(short, long, default_value_t = false)]
    display: bool,
}

pub struct Matrix {
    pub n: usize,
    pub matrix: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(n: usize) -> Matrix {
        let mut matrix = vec![vec![0.0; n]; n];
        matrix.par_iter_mut().for_each(|i| {
            //            let mut rng: StdRng = SeedableRng::from_entropy();
            let mut rng2 = rand::thread_rng();
            for j in i.iter_mut() {
                *j = rng2.gen_range(0.0..100.0);
            }
        });
        Matrix { n, matrix }
    }

    pub fn update(&mut self, index: usize) {
        let index_row = &self.matrix[index].clone();
        let _ = &self.matrix[index + 1..].par_iter_mut().for_each(|i| {
            let multiplier = i[index] / index_row[index];
            for (j, k) in i.iter_mut().zip(index_row.iter()) {
                *j = *j - multiplier * *k;
            }
        });
    }
}

fn main() {
    let dt1 = Local::now();

    let args = Args::parse();
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap();
    let mut matrix = Matrix::new(args.size);
    if args.display == true {
        matrix_render(&matrix);
    }
    for i in 0..args.size {
        matrix.update(i);
    }
    if args.display == true {
        matrix_render(&matrix);
    }
    let dt2 = Local::now();
    println!(
        "It took {:?} milliseconds to run",
        (dt2.sub(dt1)).num_milliseconds()
    );
}

fn matrix_render(matrix: &Matrix) {
    let mut output = String::with_capacity(16 * matrix.n * (matrix.n + 1));
    for i in 0..matrix.n {
        for j in 0..matrix.n {
            let s = format!("{: >9.2}", matrix.matrix[i][j]);
            output.push_str(&s);
            output.push(' ');
        }
        output.push_str("\n");
    }
    output.push_str("\n\n\n");
    print!("{}", output);
}
