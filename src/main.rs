use crate::number::extract_numbers;
//fn main() {
  //  println!("Hello, world!");
     
//use std::env;

  //  let args: Vec<String> = env::args().collect();
    //if args.len() < 3 {
      //  eprintln!("Usage: {} <enable_fib> <max_threshold>", args[1]);
        //return;
    //}

    //let enable_fib = &args[1];
    //let max_threshold = &args[2];

    //println!("\n enable_fib: {}", enable_fib);
    //println!("\n max_threshold: {}", max_threshold);

    //if enable_fib == "true" {
        // Your Fibonacci logic here
      //  println!("\n Fibonacci program is enabled with max threshold: {}", max_threshold);
    //} else {
      //  println!("\n Fibonacci program is disabled");
    //}
    
    fn main() {
        let sample_content = "This is a sample string with numbers 12; 13 , 02 02abcu1a-23egtj2,--12-29 in it.";
        let numbers = extract_numbers(sample_content);
        println!("{:?}", numbers);
        
    
    use crate::fib::fibonacci_iterative;


    let sample_content = "This is a sample string with numbers 12; 13 , 02 02abcu1a-23egtj2,--12-29 in it.";
    let numbers = extract_numbers(sample_content);
    println!("{:?}", numbers);

    let n = 100; // Example input
    println!("Fib of {}: {}", n, fibonacci_iterative(n));
    
}


mod number;
mod fib;
    
//} 