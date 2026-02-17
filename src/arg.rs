#[derive(PartialEq)]
pub struct Arguments {
    pub no_tui: bool,
    pub location: String,
    pub help: bool,
}

pub fn parse_args() -> Arguments {
    let mut it = std::env::args().skip(1); // skip program name
    let mut no_tui = false;
    let mut location = String::from("Örebro");
    let mut help = false;
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-l" => {
                // use next if it exists and parses as i32, else default to 12
                location = it
                    .next()
                    .expect("No location was given after the \"-l\" flag.")
            }
            "help" => {
                help = true;
            }
            "-t" => {
                no_tui = true;
            }
            _ => {}
        }
    }
    Arguments {
        no_tui,
        location,
        help,
    }
}
