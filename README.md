NAME

        speedreader - Read texts via RSVP

INSTALLATION:

        cargo install --git https://github.com/c3c3l14/speedreader/

SYNOPSIS:

        speedreader [FLAGS] ...

DESCRIPTION:

        A Rust based terminal rapid serial visual presentstion (RSVP)
        speedreading app. Flashes a word or short group of words from
        a text one at a time in the center of the screen in order to 
        aid the user in reading text without moving their eyes.
            
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
                                (only accepts regular terminal colors)
