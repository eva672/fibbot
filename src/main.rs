use std::env;

use octocrab::{models::{repos::DiffEntry, pulls::PullRequest, repos::Content}, Octocrab, Page};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error >>{
    let github_repository = env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "eva672/fibbot".to_string());
    let github_repository=  github_repository.split("/").collect::<Vec<&str>>();
    let owner = github_repository[0];
    let repo = github_repository[1];

let pr = octocrab::instance().pulls( owner,  repo).list_files(1).await?;
println!("{:?}", pr);
let path = &pr.items.first().unwrap().patch.clone().unwrap();
let numbers= extract_numbers::extract_numbers(&path);
println!("{:?}",numbers);
let result=main2();
println!("{:?}",result);

Ok(()) 

}
fn main2() {
    println!("Hello, world!");
    use std::env;

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <enable_fib> <max_threshold>", args[1]);
        return;
    }

    let enable_fib = &args[1];
    let max_threshold = &args[2];

    println!("\n enable_fib: {}", enable_fib);
    println!("\n max_threshold: {}", max_threshold);

    if enable_fib == "true" {
        // Your Fibonacci logic here
        println!("\n Fibonacci program is enabled with max threshold: {}", max_threshold);
    } else {
        println!("\n Fibonacci program is disabled");
        
    }
    
 functions
    

master
} 



mod extract_numbers;
mod pull_request;

