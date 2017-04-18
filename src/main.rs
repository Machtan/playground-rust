extern crate playground;

use std::env;
use std::iter;
use playground::{ArgDef, parse, ParseStatus, default_help_interrupt, default_version_interrupt};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    
    let mut first = String::new();
    let mut second = String::new();
    let mut third = false;
    let mut cool_thing: Option<String> = None;
    let mut stars = 0;
    let mut verbose = false;
    let mut numbers: Vec<i32> = Vec::new();
    
    let description = "
        Test program for an argument parsing library.
        
        Showcases the features of the library.
    ";
    
    let res = match parse(&args, vec![
        ArgDef::pos("first", &mut first)
            .help("The first argument."),
        ArgDef::pos("second", &mut second)
            .help("The second argument."),
        ArgDef::pos("third-is-better", &mut third)
            .help("Whether the third argument is better than the rest."),
        ArgDef::trail("numbers", true, &mut numbers)
            .help("A bunch of numbers used for nefarious machinations."),
        
        ArgDef::option("cool", &mut cool_thing)
            .help("Something that you think is cool enough to pass."),
        ArgDef::count("star", &mut stars).short("s")
            .help("How many stars does this library deserve?"),
        ArgDef::flag("verbose", &mut verbose).short("v")
            .help("Print as much information as possible."),
        
        default_help_interrupt(description).short("h"),
        default_version_interrupt(),
    ]) {
        Ok(res) => res,
        Err((msg, help)) => {
            println!("Parse failed: {}", msg);
            help.print_usage();
            return;
        }
    };
    
    match res {
        ParseStatus::Success => {}
        ParseStatus::Interrupted(name) => {
            println!("Parse interrupted: <{}>", name);
            return;
        }
    }
    
    println!("First:   {}", first);
    println!("Second:  {}", second);
    println!("Third is better?:   {}", third);
    println!("");
    println!("Numbers: {:?}", numbers);
    if verbose {
        println!("VERBOSE!");
    }
    if let Some(cool) = cool_thing {
        println!("Got a real cool {} :u!", cool);
    } else {
        println!("Nothing's cool anymore");
    }
    println!("Library rating: {}", iter::repeat('*').take(stars).collect::<String>());
}
