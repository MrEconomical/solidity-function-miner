// Mining config parameters

pub struct Config {
    pub name: String,
    pub params: String,
    pub target: u32,
    pub threads: u32
}

impl Config {
    // Parse command line arguments

    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Get function name

        args.next();
        let name = args.next()
            .ok_or("no function name specified")?;
        
        // Get function parameters

        let params = match args.next() {
            Some(arg) => {
                if !arg.starts_with("(") || !arg.ends_with(")") {
                    return Err("expected function parameters in format (type1,type2,...)");
                } else if arg.contains(char::is_whitespace) {
                    return Err("function parameters must be in format (type1,type2,...) without spaces");
                }
                arg
            },
            None => return Err("no function parameters specified")
        };

        // Get zero byte target

        let target = match args.next() {
            Some(arg) => {
                match arg.parse::<u32>() {
                    Ok(value) => {
                        if value > 4 {
                            return Err("zero byte target must be less than or equal to 4");
                        }
                        value
                    },
                    Err(_) => return Err("could not parse zero byte target as integer")
                }
            },
            None => return Err("no zero byte target specified")
        };

        // Get thread count

        let threads = match args.next() {
            Some(arg) => {
                match arg.parse::<u32>() {
                    Ok(value) => {
                        if value == 0 {
                            return Err("thread count must be larger than 0")
                        }
                        value
                    },
                    Err(_) => return Err("could not parse thread count as integer")
                }
            },
            None => return Err("no thread count specified")
        };

        Ok(Config { name, params, target, threads })
    }
}