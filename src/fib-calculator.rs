pub fn fibonacci_iterative(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let new_curr = prev + curr;
        prev = curr;
        curr = new_curr;
    }

    curr
}
