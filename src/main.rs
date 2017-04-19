extern crate playground;

use std::env;
use std::iter;
use playground::{ArgDef, parse, ParseError, default_help_interrupt, default_version_interrupt};
use std::process;

fn main() {
    if let Some(exit_code) = epub_main() {
        process::exit(exit_code);
    }
}

#[allow(unused)]
fn epub_main() -> Option<i32> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let description = "
        Program to create ePub e-book files.
    ";
    
    match parse(&args, vec![
        ArgDef::cmd("create", |args| {
            parse(args, vec![])?;
            Ok(())
        })
        .help("Creates a new ePub from a given specification."),
        
        ArgDef::cmd("example", |args| {
            parse(args, vec![])?;
            Ok(())
        })
        .help("Prints a template for an ePub specification file."),
        
        ArgDef::cmd("from_folder", |args| {
            let mut folder = String::new();
            parse(args, vec![
                ArgDef::pos("folder", &mut folder)
                    .help("The folder to load images from."),
                
                default_help_interrupt("epub from_folder", 
                "
                    Creates a simple epub from the images in the given folder.
                    This is useful for quickly creating rather bad comic epubs.
                "),
            ])?;
            
            println!("Creating epub from folder '{}'...", folder);
            
            Ok(())
        })
        .help("Creates a simple ePub from the images in the given folder."),
        
        default_help_interrupt("epub", description).short("h"),
        default_version_interrupt(),
    ]) {
        Ok(_) => {},
        Err(ParseError::Interrupted(name)) => {
            println!("Parse interrupted: <{}>", name);
            return None;
        }
        Err(ParseError::InvalidDefinitions(msg)) => {
            panic!(msg);
        }
        // PROBLEM: Usage is not shown for subcommands.
        // I should make a way of handling it in the subcommand or passing through.
        Err(ParseError::ParseFailed(msg, help)) => {
            println!("Parse failed: {}", msg);
            help.print_usage("epub");
            return Some(1);
        }
    };
    
    None
}

#[allow(unused)]
fn argonaut_main() -> Option<i32> {
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
    
    match parse(&args, vec![
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
        
        default_help_interrupt("argonaut", description).short("h"),
        default_version_interrupt(),
    ]) {
        Ok(_) => {},
        Err(ParseError::Interrupted(name)) => {
            println!("Parse interrupted: <{}>", name);
            return None;
        }
        Err(ParseError::InvalidDefinitions(msg)) => {
            panic!(msg);
        }
        Err(ParseError::ParseFailed(msg, help)) => {
            println!("Parse failed: {}", msg);
            help.print_usage("argonaut");
            return Some(1);
        }
    };
    
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
    
    None
}
