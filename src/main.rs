use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo, style::Print};
use std::env;
use std::io::BufRead;
use std::thread;
use std::time::Duration;
use atty::Stream;

#[derive(Debug, Copy, Clone)]
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

fn get_center_of_terminal_offset() -> (u16, u16) {
    let (x, y) = crossterm::terminal::size().unwrap();
    ((x / 2), (y / 2))
}

fn print_at_center(text: &str) {
    let (x, y) = get_center_of_terminal_offset();

    //setup stdout for crossterm
    let mut stdout = std::io::stdout();
    
    //clear the screen
    execute!(stdout, Clear(ClearType::All));
    
    if text.len() > x as usize {
        //truncate text if it is too long
        let mut text_clean = text.to_string();
        text_clean.truncate((x*2) as usize);
        execute!(stdout, MoveTo(x - (text_clean.len() / 2) as u16, y), Print(text_clean));
    } else {
        execute!(stdout, MoveTo(x - (text.len() / 2) as u16, y), Print(text));
    }
    
}

fn get_word(text: &str, word_number: u32) -> String {
    let mut words = text.split_whitespace();
    let mut current_word = 0;
    let mut word = String::new();
    while let Some(w) = words.next() {
        if current_word == word_number {
            word = w.to_string();
            break;
        }
        current_word += 1;
    }
    word
}

fn get_word_count(text: &str) -> u32 {
    text.split_whitespace().count() as u32
}

fn main() {
    let version = "b0.1.042";
    let help = "speedreader v0.0.000
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
                            (only accepts regular terminal colors)";
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
    let mut highlight = Colors::White;
    let mut stdin = String::new();

    // basic error handling
    // if there is no stdin then print help message
    if atty::is(Stream::Stdin) {
        println!("{}", help);
        return;
    }
    //get stdin as string
    println!("loading text from stdin...");
    while let Ok(n) = std::io::stdin().read_line(&mut stdin) {
        if n == 0 {
            break;
        }
    }
    stdin = stdin.replace("\n", " ");
    stdin = stdin.replace("\r", " ");



    // argument parsing, see below for flag usage
    let args: Vec<String> = env::args().collect();
    // let argumentnamegoeshere = args.iter().any(|arg| arg == "flaggoeshere");
    if args.iter().any(|arg| arg == "-h") {
        println!("{}", help);
        return;
    }
    if args.iter().any(|arg| arg == "-v") {
        println!("speedreader v{}\nby c3c3l14 :3", version);
        return;
    }

    if args.iter().any(|arg| arg == "-wpm") {
        let index = args.iter().position(|arg| arg == "-wpm").unwrap();
        wpm = args.get(index + 1).unwrap().parse::<u32>().unwrap();
    }

    if args.iter().any(|arg| arg == "-f") {
        println!("Unimplemented sowwy use stdin");
        unimplemented!();
    }

    if args.iter().any(|arg| arg == "-w") {
        unimplemented!();
        if args.iter().any(|arg| arg == "-c") {
            println!("Cannot use both -w and -c arguments");
            return;
        }
        let index = args.iter().position(|arg| arg == "-w").unwrap();
        words = args.get(index + 1).unwrap().parse::<u32>().unwrap();
        
    }

    if args.iter().any(|arg| arg == "-c") {
        unimplemented!();
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
        unimplemented!();
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
        unimplemented!();
        // if -highlight used then set highlight to user specified color
        // if -highlight used but no following argument starts with - or is invalid then set highlight to red
        let index = args.iter().position(|arg| arg == "-highlight").unwrap();
        
        if args.get(index + 1).is_none() || args.get(index + 1).unwrap().starts_with("-") {
            highlight = Colors::Red;
        } else {
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

    // println!("{}", wpm);
    // println!("{}", words); 
    // println!("{}", characters);
    // println!("{}", pause);
    // println!("{:?}", color);
    // println!("{:?}", highlight);
    let (testx, testy) = get_center_of_terminal_offset();
    // println!("{}, {}", testx, testy);
    

    //begin actual program loop
    
    //disable cursor
    execute!(std::io::stdout(), crossterm::cursor::Hide).unwrap();

    for current_word in 1..=get_word_count(&stdin) {
        let delay = Duration::from_millis((60000 / wpm).into());
        
        print_at_center(get_word(&stdin, current_word).as_str());
        if get_word(&stdin, current_word).chars().any(|c| c == '.' || c == '!' || c == '?') {
            thread::sleep(delay * pause as u32);
        }

        thread::sleep(delay);
    }

    return;
}

