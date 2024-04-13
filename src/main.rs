#[derive(Debug)]
enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

fn main() {
    // default values/init vars
    //
    // the type specification is unnecessary, 
    // but I like to be explicit and compiler 
    // optimizations will remove it anyway
    let mut wpm: u32 = 250;
    let mut words: u32 = 1;
    let mut characters: u32 = 12;
    let mut pause: f32 = 1.0;
    let mut color = Colors::White;
    let mut highlight = Colors::Red;


    // argument parsing, see below for flag usage
    use std::env;
    let args: Vec<String> = env::args().collect();
    // let argumentnamegoeshere = args.iter().any(|arg| arg == "flaggoeshere");
    if args.iter().any(|arg| arg == "-h") {
        println!("speedreader v0.0.000
        A simple terminal rapid serial visual presentation speed reader
        
        USAGE:
            speedreader [FLAGS] [OPTIONS]
            
        FLAGS:
            -h                  Prints help information
            -v                  Prints version information
            -f <filename>       Sets file to read (otherwise read from stdin)
            -w <number>         Sets number of words to read at a time 
                                (default is 1) (incompatible with -c)

            -c <number>         Sets number of characters to read at a time 
                                (default is 12) (incompatible with -w)

            -p <number>         Sets length of pause after punctuation 
                                (default is 1 word length)

            -color <color>      Sets color of text (default is white) 
                                (only accepts regular terminal colors)

            -highlight <color>  Sets color of center char 
                                (default if -highlight is not used is same
                                color as text. default if -highlight is used,
                                but no color is specified, is red) 
                                (only accepts regular terminal colors)");
        return;
    }
    let version = args.iter().any(|arg| arg == "-v");
    if args.iter().any(|arg| arg == "-wpm") {
        let index = args.iter().position(|arg| arg == "-wpm").unwrap();
        wpm = args.get(index + 1).unwrap().parse::<u32>().unwrap();
    }

    if args.iter().any(|arg| arg == "-f") {
        println!("Unimplemented sowwy use stdin");
        unimplemented!();
    }

    if args.iter().any(|arg| arg == "-w") {
        if args.iter().any(|arg| arg == "-c") {
            println!("Cannot use both -w and -c arguments");
            return;
        }
        let index = args.iter().position(|arg| arg == "-w").unwrap();
        words = args.get(index + 1).unwrap().parse::<u32>().unwrap();
        
    }

    if args.iter().any(|arg| arg == "-c") {
        if args.iter().any(|arg| arg == "-w") {
            println!("Cannot use both -w and -c arguments");
            return;
        }
        let index = args.iter().position(|arg| arg == "-c").unwrap();
        characters = args.get(index + 1).unwrap().parse::<u32>().unwrap();
    }
    if args.iter().any(|arg| arg == "-p") {
        let index = args.iter().position(|arg| arg == "-p").unwrap();
        pause = args.get(index + 1).unwrap().parse::<f32>().unwrap();
    }
    if args.iter().any(|arg| arg == "-color") {
        let index = args.iter().position(|arg| arg == "-color").unwrap();
        color = match args.get(index + 1).unwrap().as_str() {
            "black" => Colors::Black,
            "red" => Colors::Red,
            "green" => Colors::Green,
            "yellow" => Colors::Yellow,
            "blue" => Colors::Blue,
            "magenta" => Colors::Magenta,
            "cyan" => Colors::Cyan,
            "white" => Colors::White,
            _ => {
                println!("Invalid color");
                return;
            }
        };
        highlight = color;
    }
    if args.iter().any(|arg| arg == "-highlight") {
        // if -highlight used then set highlight to user specified color
        // if -highlight used but no following argument starts with - or is empty then set highlight to red
        let index = args.iter().position(|arg| arg == "-highlight").unwrap();
        highlight = match args.get(index + 1).unwrap().as_str() {
            "black" => Colors::Black,
            "red" => Colors::Red,
            "green" => Colors::Green,
            "yellow" => Colors::Yellow,
            "blue" => Colors::Blue,
            "magenta" => Colors::Magenta,
            "cyan" => Colors::Cyan,
            "white" => Colors::White,
            _ => {
                println!("Invalid color");
                return;
            }
        };
        if args.get(index + 1).unwrap().starts_with("-") || args.get(index + 1).unwrap().is_empty() {
            highlight = Colors::Red;
        }
    }


    // flags:
    // -h: display help message
    // -v: display version
    // -wpm <number>: set words per minute
    // -f <filename>: set file to read (otherwise read from stdin)
    // -w <number>: set number of words to read at a time (default is 1) (incompatible with -c)
    // -c <number>: set number of characters to read at a time (default is 12) (incompatible with -w)
    // -p <number>: set length of pause after punctuation (default is 1 word length)
    // -color <color>: set color of text (default is white) (only accepts regular terminal colors)
    // -highlight <color>: set color of center char (default if -highlight is not provided is same color as text. default if -highlight is provided but no color is specified is red) (only accepts regular terminal colors)

    println!("{}", wpm);
    println!("{}", words); 
    println!("{}", characters);
    println!("{}", pause);
    println!("{:?}", color);
    println!("{:?}", highlight);

    return;
}

