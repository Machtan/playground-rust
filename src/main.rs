
use std::borrow::{Cow, Borrow};
use std::rc::Rc;
use std::fmt::Debug;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;

/*
List of parse descriptions
- Nargs (argtype)
- Target (which type)
- Converter (Parse implementation for type?)

They must be typeless (?)
And at the same time generic over the target type

let mut blub: Vec<i32>;
Desc::new(&mut blub).help("blah").nargs(3);

impl<T> ArgTarget for T where T: Parse {
    fn parse<I: Iterator<Item=&str>>(args: I, req_nargs: usize) {
        
    }
}

Also some kind of 'subcommand' parser
Basically a map-lookup with a default error message + version + help

*/

#[derive(Debug)]
pub struct Help<'def> {
    descriptions: Vec<HelpDesc<'def>>,
}

impl<'def> Help<'def> {
    pub fn from_definitions<'tar>(definitions: &[Desc<'def, 'tar>]) -> Help<'def> {
        let descriptions: Vec<_> = definitions.iter().map(|d| HelpDesc::from(d)).collect();
        Help { descriptions }
    }
}

#[derive(Debug)]
struct HelpDesc<'def> {
    name: Cow<'def, str>,
    desc: Option<Cow<'def, str>>,
    kind: HelpDescKind<'def>,
}

#[derive(Debug)]
enum HelpDescKind<'def> {
    Pos,
    Trail(bool),
    Switch(Option<Cow<'def, str>>),
    Count(Option<Cow<'def, str>>),
    Value(Option<Cow<'def, str>>),
    Interrupt(Option<Cow<'def, str>>),
}

impl<'def> HelpDesc<'def> {
    fn from<'target>(desc: &Desc<'def, 'target>) -> HelpDesc<'def> {
        use self::HelpDescKind::*;
        let kind = match desc.kind {
            DescKind::Positional { .. } => Pos,
            DescKind::Trail { optional, .. } => Trail(optional),
            DescKind::Switch { ref short, .. } => Switch(short.clone()),
            DescKind::Count { ref short, .. } => Count(short.clone()),
            DescKind::Value { ref short, .. } => Value(short.clone()),
            DescKind::Interrupt { ref short, .. } => Interrupt(short.clone()),
        };
        HelpDesc {
            name: desc.name.clone(),
            desc: desc.help_desc.clone(),
            kind: kind,
        }
    }
}

pub trait SingleTarget: Debug {
    fn parse(&mut self, value: &str) -> Result<(), String>;
}

impl SingleTarget for String {
    fn parse(&mut self, value: &str) -> Result<(), String> {
        self.clear();
        self.push_str(value);
        Ok(())
    }
}

pub trait TrailTarget: Debug {
    fn parse_and_add(&mut self, value: &str) -> Result<(), String>;
}

#[derive(Debug)]
pub struct Desc<'def, 'tar> {
    name: Cow<'def, str>,
    kind: DescKind<'def, 'tar>,
    help_desc: Option<Cow<'def, str>>,
}

#[derive(Debug)]
enum DescKind<'def, 'tar> {
    Positional { 
        target: &'tar mut SingleTarget,
    },
    Trail { 
        optional: bool, 
        target: &'tar mut TrailTarget,
    },
    Switch {
        short: Option<Cow<'def, str>>,
        target: &'tar mut bool,
    },
    Count {
        short: Option<Cow<'def, str>>,
        target: &'tar mut usize,
    },
    Value {
        short: Option<Cow<'def, str>>,
        target: &'tar mut SingleTarget,
    },
    Interrupt {
        short: Option<Cow<'def, str>>,
        callback: fn(Rc<Help<'def>>),
    },
}

// TODO: Make 'short'-setting safe by using an Into<> pattern.
impl<'def, 'tar> Desc<'def, 'tar> {
    fn new<N>(name: N, kind: DescKind<'def, 'tar>) -> Desc<'def, 'tar> 
      where N: Into<Cow<'def, str>> 
    {
        Desc {
            name: name.into(),
            kind: kind,
            help_desc: None,
        }
    }
    
    pub fn pos<N>(name: N, target: &'tar mut SingleTarget) -> Desc<'def, 'tar> 
      where N: Into<Cow<'def, str>> 
    {
        Desc::new(name, DescKind::Positional { target })
    }
    
    pub fn interrupt<N>(name: N, callback: fn(Rc<Help<'def>>)) -> Desc<'def, 'tar>
      where N: Into<Cow<'def, str>> 
    {
        Desc::new(name, DescKind::Interrupt { short: None, callback })
    }
    
    pub fn count<N>(name: N, target: &'tar mut usize) -> Desc<'def, 'tar>
      where N: Into<Cow<'def, str>> 
    {
        Desc::new(name, DescKind::Count { short: None, target })
    }
    
    pub fn short<N>(mut self, short: N) -> Self where N: Into<Cow<'def, str>> {
        use self::DescKind::*;
        self.kind = match self.kind {
            Positional { .. } | Trail { .. } => {
                panic!("Positional and trail arguments cannot have a short identified");
            },
            Switch { target, .. } => Switch { short: Some(short.into()), target },
            Count { target, .. } => Count { short: Some(short.into()), target },
            Value { target, .. } => Value { short: Some(short.into()), target },
            Interrupt { callback, .. } => Interrupt { short: Some(short.into()), callback },
        };
        self
    }
}

#[derive(Debug)]
enum OptTarget<'def, 'tar> {
    Switch(&'tar mut bool),
    Count(&'tar mut usize),
    Value(&'tar mut SingleTarget),
    Interrupt(fn(Rc<Help<'def>>)),
}

#[derive(Debug)]
struct Defs<'def, 'tar> {
    positional: VecDeque<(Cow<'def, str>, &'tar mut SingleTarget)>,
    // (satisfied, target)
    trail: Option<(Cow<'def, str>, bool, &'tar mut TrailTarget)>,
    options: HashMap<Cow<'def, str>, OptTarget<'def, 'tar>>,
    short_map: HashMap<Cow<'def, str>, Cow<'def, str>>,
}

impl<'def, 'tar> Defs<'def, 'tar> {
    fn get_interned_name(&self, option: &str) -> Cow<'def, str> {
        self.options.keys().find(|k| k.as_ref() == option).unwrap().clone()
    }
    
    fn get_target<'a>(&'a mut self, option: &str) -> Result<(Cow<'def, str>, &'a mut OptTarget<'def, 'tar>), String> {
        let mut key = &option[2..];
        if ! option.starts_with("--") {
            if let Some(mapped_key) = self.short_map.get(&option[1..]) {
                key = mapped_key.as_ref();
            } else {
                return Err(format!("Unknown option: '{}'", option));
            }
        }
        let name = self.get_interned_name(key);
        if let Some(target) = self.options.get_mut(key) {
            Ok((name, target))
        } else {
            Err(format!("Unknown option: '{}'", option))
        }
    }
    
    
    fn read_option<'arg, I>(&mut self, option: &str, args: &mut I, 
        given_values: &mut HashSet<Cow<'def, str>>, help: Rc<Help<'def>>) 
        -> Result<Option<Cow<'def, str>>, String>
      where I: Iterator<Item=&'arg str>
    {
        use self::OptTarget::*;
        match self.get_target(option)? {
            (_, &mut Switch(ref mut flag)) => **flag = true,
            (_, &mut Count(ref mut count)) => **count += 1,
            (ref name, &mut Value(ref mut value)) => {
                if given_values.contains(name) {
                    return Err(format!("Option '{}' given twice!", name));
                }
                let arg = if let Some(arg) = args.next() {
                    arg
                } else {
                    return Err(format!("Missing argument for option '{}'", option));
                };
                value.parse(arg);
                given_values.insert(name.clone());
            }
            (name, &mut Interrupt(callback)) => {
                callback(help);
                return Ok(Some(Cow::Owned(String::from(name))));
            }
        }
        Ok(None)
    }
}

fn parse_definitions<'def, 'tar>(defs: Vec<Desc<'def, 'tar>>) -> Result<Defs<'def, 'tar>, String> {
    let mut positional = VecDeque::new();
    let mut trail = None;
    let mut options = HashMap::new(); // long-to-arg
    let mut short_map = HashMap::new(); // short-to-long
    for desc in defs {
        match desc.kind {
            DescKind::Positional { target } => {
                positional.push_back((desc.name, target));
            }
            DescKind::Trail { optional, target } => {
                if trail.is_some() {
                    return Err(format!("Two trails defined."));
                }
                trail = Some((desc.name, optional, target));
            }
            DescKind::Switch { short, target } => {
                if let Some(short) = short {
                    if short_map.contains_key(&short) {
                        return Err(format!("Short name '{}' defined twice.", short));
                    }
                    short_map.insert(short, desc.name.clone());
                }
                if options.contains_key(&desc.name) {
                    return Err(format!("Option '{}' defined twice.", desc.name));
                }
                options.insert(desc.name, OptTarget::Switch(target));
            }
            DescKind::Count { short, target } => {
                if let Some(short) = short {
                    if short_map.contains_key(&short) {
                        return Err(format!("Short name '{}' defined twice.", short));
                    }
                    short_map.insert(short, desc.name.clone());
                }
                if options.contains_key(&desc.name) {
                    return Err(format!("Option '{}' defined twice.", desc.name));
                }
                options.insert(desc.name, OptTarget::Count(target));
            }
            DescKind::Value { short, target } => {
                if let Some(short) = short {
                    if short_map.contains_key(&short) {
                        return Err(format!("Short name '{}' defined twice.", short));
                    }
                    short_map.insert(short, desc.name.clone());
                }
                if options.contains_key(&desc.name) {
                    return Err(format!("Option '{}' defined twice.", desc.name));
                }
                options.insert(desc.name, OptTarget::Value(target));
            }
            DescKind::Interrupt { short, callback } => {
                if let Some(short) = short {
                    if short_map.contains_key(&short) {
                        return Err(format!("Short name '{}' defined twice.", short));
                    }
                    short_map.insert(short, desc.name.clone());
                }
                if options.contains_key(&desc.name) {
                    return Err(format!("Option '{}' defined twice.", desc.name));
                }
                options.insert(desc.name, OptTarget::Interrupt(callback));
            }
        }
    }
    Ok(Defs { positional, trail, options, short_map })
}

/*
DESIGN: Do I wait with assigning values until all arguments have been 'satisfied'?
Or do I just start parsing/assigning as soon as possible so that bad arguments
are caught faster?
For now it'll be 2, since that seems simpler
*/

#[must_use = "If the parse was interrupted, the parse variables will be in an unknown state"]
#[derive(Debug)]
pub enum ParseStatus<'def> {
    Success,
    Interrupted(Cow<'def, str>),
}

pub fn parse<'def, 'tar, T>(args: &[T], definitions: Vec<Desc<'def, 'tar>>) -> Result<ParseStatus<'def>, (String, Rc<Help<'def>>)>
  where T: Borrow<str> { 
    let help = Rc::new(Help::from_definitions(&definitions));
    let mut defs = match parse_definitions(definitions) {
        Ok(defs) => defs,
        Err(msg) => return Err((msg, help)),
    };
    println!("Defs: {:?}", defs);
    let mut args = args.iter().map(|e| e.borrow());
    // value-type definitions that have been given and should not be overridden
    let mut given_values = HashSet::new();
    
    while let Some(arg) = args.next() {
        if arg.starts_with("-") {
            let opt_res = match defs.read_option(arg, &mut args, 
                    &mut given_values, help.clone()) {
                Ok(res) => res,
                Err(msg) => return Err((msg, help)),
            };
            if let Some(interrupt) = opt_res {
                return Ok(ParseStatus::Interrupted(interrupt));
            }
        } else if ! defs.positional.is_empty() {
            let (_name, target) = defs.positional.pop_front().unwrap();
            match target.parse(arg) {
                Ok(()) => {},
                Err(msg) => return Err((msg, help)),
            } // TODO: chain err
        } else {
            if let Some((_, ref mut satisfied, ref mut target)) = defs.trail {
                match target.parse_and_add(arg) {
                    Ok(()) => {},
                    Err(msg) => return Err((msg, help)),
                }; // TODO: chain err
                *satisfied = true;
            } else {
                return Err((format!("Unexpected argument '{}'", arg), help));
            }            
        }
    }
    if let Some((name, _)) = defs.positional.pop_front() {
        return Err((format!("Missing positional argument '{}'", name), help));
    }
    if let Some((name, satisfied, _)) = defs.trail {
        if ! satisfied {
            return Err((format!("Expected at least one trailing argument for '{}'", name), help))
        }
    }
    Ok(ParseStatus::Success)
}

fn print_help<'def>(help: Rc<Help<'def>>) {
    println!("HELP ME!");
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    
    let mut first = String::new();
    let mut second = String::new();
    let mut count = 0;
    
    let res = match parse(&args, vec![
        Desc::pos("first", &mut first),
        Desc::pos("second", &mut second),
        Desc::interrupt("help", print_help).short("h"),
        Desc::count("count", &mut count).short("c"),
    ]) {
        Ok(res) => res,
        Err((msg, help)) => {
            println!("Parse failed: {}", msg);
            print_help(help);
            return;
        }
    };
    
    match res {
        ParseStatus::Success => {}
        ParseStatus::Interrupted(name) => {
            return;
        }
    }
    
    println!("First:  {}", first);
    println!("Second: {}", second);
    println!("Count:  {}", count);
}
