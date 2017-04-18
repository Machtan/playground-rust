use argdef::{SingleTarget, CollectionTarget, OptionTarget, ArgDef, ArgDefKind};
use help::Help;
use std::collections::{HashMap, HashSet, VecDeque};
use std::borrow::{Cow, Borrow};
use std::rc::Rc;

/// References to the targets of non-positional arguments.
//#[derive(Debug)]
pub enum TargetRef<'def, 'tar> {
    Flag(&'tar mut bool),
    Count(&'tar mut usize),
    OptArg(&'tar mut OptionTarget),
    Interrupt(Box<FnMut(Rc<Help<'def>>)>),
}

/// Sorted argument definitions. Updated mutably during the parse.
//#[derive(Debug)]
pub struct ParseState<'def, 'tar> {
    positional: VecDeque<(Cow<'def, str>, &'tar mut SingleTarget)>,
    // (satisfied, target)
    trail: Option<(Cow<'def, str>, bool, &'tar mut CollectionTarget)>,
    options: HashMap<Cow<'def, str>, TargetRef<'def, 'tar>>,
    short_map: HashMap<Cow<'def, str>, Cow<'def, str>>,
}

impl<'def, 'tar> ParseState<'def, 'tar> {
    /// Returns the internal object representing the given option name.
    fn get_interned_name(&self, option: &str) -> Cow<'def, str> {
        self.options.keys().find(|k| k.as_ref() == option).unwrap().clone()
    }
    
    /// Attempts to find a target from the given option.
    fn get_target<'a>(&'a mut self, option: &str) -> Result<(Cow<'def, str>, &'a mut TargetRef<'def, 'tar>), String> {
        let mut key = &option[2..];
        if ! option.starts_with("--") {
            if let Some(mapped_key) = self.short_map.get(&option[1..]) {
                key = mapped_key.as_ref();
            } else {
                return Err(format!("Unknown option: '{}'", option));
            }
        }
        if ! self.options.contains_key(key) {
            return Err(format!("Unknown option '{}'", option));
        }
        // INVARIANT: key is contained
        let name = self.get_interned_name(key);
        let target = self.options.get_mut(key).unwrap();
        Ok((name, target))
    }
    
    
    fn read_option<'arg, I>(&mut self, option: &str, args: &mut I, 
        given_values: &mut HashSet<Cow<'def, str>>, help: Rc<Help<'def>>) 
        -> Result<Option<Cow<'def, str>>, String>
      where I: Iterator<Item=&'arg str>
    {
        use self::TargetRef::*;
        match self.get_target(option)? {
            (_, &mut Flag(ref mut flag)) => {
                **flag = true;
            }
            (_, &mut Count(ref mut count)) => {
                **count += 1;
            }
            (ref name, &mut OptArg(ref mut value)) => {
                if given_values.contains(name) {
                    return Err(format!("Option '{}' given twice!", name));
                }
                let arg = if let Some(arg) = args.next() {
                    arg
                } else {
                    return Err(format!("Missing argument for option '{}'", option));
                };
                value.parse(arg)?;
                given_values.insert(name.clone());
            }
            (ref name, &mut Interrupt(ref mut callback)) => {
                callback(help);
                return Ok(Some(name.clone()));
            }
        }
        Ok(None)
    }
}

/// Sorts the given definitions and checks that all invariants are upheld.
pub fn parse_definitions<'def, 'tar>(defs: Vec<ArgDef<'def, 'tar>>) 
        -> Result<ParseState<'def, 'tar>, String> {
    let mut positional = VecDeque::new();
    let mut trail = None;
    let mut options = HashMap::new(); // long-to-arg
    let mut short_map = HashMap::new(); // short-to-long
    for desc in defs {
        match desc.kind {
            ArgDefKind::Positional { target } => {
                positional.push_back((desc.name, target));
            }
            ArgDefKind::Trail { optional, target } => {
                if trail.is_some() {
                    return Err(format!("Two trails defined."));
                }
                trail = Some((desc.name, optional, target));
            }
            ArgDefKind::Flag { short, target } => {
                if let Some(short) = short {
                    if short_map.contains_key(&short) {
                        return Err(format!("Short name '{}' defined twice.", short));
                    }
                    short_map.insert(short, desc.name.clone());
                }
                if options.contains_key(&desc.name) {
                    return Err(format!("Option '{}' defined twice.", desc.name));
                }
                options.insert(desc.name, TargetRef::Flag(target));
            }
            ArgDefKind::Count { short, target } => {
                if let Some(short) = short {
                    if short_map.contains_key(&short) {
                        return Err(format!("Short name '{}' defined twice.", short));
                    }
                    short_map.insert(short, desc.name.clone());
                }
                if options.contains_key(&desc.name) {
                    return Err(format!("Option '{}' defined twice.", desc.name));
                }
                options.insert(desc.name, TargetRef::Count(target));
            }
            ArgDefKind::OptArg { short, target } => {
                if let Some(short) = short {
                    if short_map.contains_key(&short) {
                        return Err(format!("Short name '{}' defined twice.", short));
                    }
                    short_map.insert(short, desc.name.clone());
                }
                if options.contains_key(&desc.name) {
                    return Err(format!("Option '{}' defined twice.", desc.name));
                }
                options.insert(desc.name, TargetRef::OptArg(target));
            }
            ArgDefKind::Interrupt { short, callback } => {
                if let Some(short) = short {
                    if short_map.contains_key(&short) {
                        return Err(format!("Short name '{}' defined twice.", short));
                    }
                    short_map.insert(short, desc.name.clone());
                }
                if options.contains_key(&desc.name) {
                    return Err(format!("Option '{}' defined twice.", desc.name));
                }
                options.insert(desc.name, TargetRef::Interrupt(callback));
            }
        }
    }
    Ok(ParseState { positional, trail, options, short_map })
}

/// How a parse finished.
#[must_use = "If the parse was interrupted, the parse variables will be in an unknown state"]
#[derive(Debug)]
pub enum ParseStatus<'def> {
    /// All expected arguments were parsed and assigned.
    Success,
    /// An interrupt-flag with the given name was encountered.
    /// 
    /// The arguments are in a partially-assigned state.
    Interrupted(Cow<'def, str>),
}

/// Parses the given arguments and updates the defined variables with them.
pub fn parse<'def, 'tar, T>(args: &[T], definitions: Vec<ArgDef<'def, 'tar>>) -> Result<ParseStatus<'def>, (String, Rc<Help<'def>>)>
  where T: Borrow<str> { 
    let help = Rc::new(Help::from_definitions(&definitions));
    let mut defs = match parse_definitions(definitions) {
        Ok(defs) => defs,
        Err(msg) => return Err((msg, help)),
    };
    
    //println!("Defs: {:?}", defs);
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

