use std::time::SystemTime;

fn fib(n: u128) -> u128 {
    if n <= 1 {
        return n ;
    }
    return fib(n - 1) + fib(n - 2 );
}

fn fib_dynamic(m: u128) {
    let mut fibArray = [0, m];
    fibArray[0] = 1;
    fibArray[1] = 1;

    for x in 2..m {
         fibArray[x] = fibArray[x -1] + fibArray[x-2];
    }

    return fibArray[m];






}

fn main() {

    println!("Recursive Fibonacci:");

    for n in 1..51 {
        let before = SystemTime::now();
        let result = fib(n);
        let after = SystemTime::now();
        let difference = after.duration_since(before);
        let difference = difference.expect("Did the clock go back?");
        println!("The {}-term is: {}",n, result);
        println!("Time it took: {:?}", difference);
    }
}