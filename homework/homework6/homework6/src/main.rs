mod q1;
mod q2;

use std::time::SystemTime;

fn main() {

    println!("Recursive Fibonacci:");

    for n in 1..=50 {
        let before = SystemTime::now();
        let result = q1.fib(n);
        let after = SystemTime::now();
        let difference = after.duration_since(before);
        let difference = difference.expect("Did the clock go back?");
        println!("The {}-term is: {}",n, result);
        println!("Time it took: {:?}", difference);
    }

    println!("Dynamic Fibonacci:");

    for n in 1..=50 {
        let before = SystemTime::now();
        let result = q2.fib_dynamic(n);
        let after = SystemTime::now();
        let difference = after.duration_since(before);
        let difference = difference.expect("Did the clock go back?");
        println!("The {}-term is: {}",n, result);
        println!("Time it took: {:?}", difference);
    }
}