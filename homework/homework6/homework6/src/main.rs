fn fib(n: u64) -> u64 {
    if n <= 1 {
        return n ;
    }
    return fib(n - 1) + fib(n - 2 );

}

fn main() {
    println!("Hello, world!");

    let n = 25;
    let result = fib(n);

    println!("Recursive Fibonacci: \n");
    println!("The {}-term is: {}",n, result);

}
