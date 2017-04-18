/*!
A simple argument parsing library.

# Lifetimes
`'def`: `argument definition`

The lifetime of argument definition passed to `parse`


`'tar`: `target`

The lifetime of target pointers used when defining arguments.
*/

#![feature(unicode)]
extern crate std_unicode;

mod argdef;
mod help;
mod parse;

use std::borrow::{Cow};
use std::env;
use std::iter;

use argdef::{ArgDef};
use parse::{parse, ParseStatus};

/*
DESIGN: Do I wait with assigning values until all arguments have been 'satisfied'?
Or do I just start parsing/assigning as soon as possible so that bad arguments
are caught faster?
For now it'll be 2, since that seems simpler

# option 1
read through the arguments and assign each to a matching option
if an interrupt is encountered: 
    run the callback and return the interrupt
validate each argument (add 'validate' to the interface)
go through and parse every value into its target
return success
*/

/// Creates a default help interrupt for `--help`.
pub fn default_help_interrupt<'def, 'tar, D>(description: D)
        -> ArgDef<'def, 'tar> 
  where D: Into<Cow<'static, str>>
{
    let description = description.into();
    ArgDef::interrupt("help", move |help| {
        help.print_help(description.as_ref());
    }).help("Print this message and abort.")
}

/// Creates a default version interrupt for `--version`.
pub fn default_version_interrupt<'def, 'tar>() -> ArgDef<'def, 'tar> {
    ArgDef::interrupt("version", |_| {
        println!("{}", option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0"));
    }).help("Print version string and abort.")
}

/*
Tasks
- DONE Usage generator (printer)
- DONE Help generator (printer)
- Simple subcommand abstraction
*/
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
