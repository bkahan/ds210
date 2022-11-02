pub(crate) fn fib_dynamic(m: u128) ->  &[u128] {
    let mut current_fib : [u128; 101] = [0; 101 ];
    current_fib[0] = 0;
    current_fib[1] = 1;

    match m {
        0 => current_fib[0],
        1 => current_fib[1],
        _ => {
            for _ in 2..=m  {
                print!("Hello");

            }
            return current_fib[m];
        }
    }




    match m {
        0 => 0,
        1 => 1,
        _ => {
            for _ in 2..=m {
                //current_fib = (current_fib.1, current_fib.0 + current_fib.1)
            }
            //return current_fib.1;
            return 1;
        }
    }
}