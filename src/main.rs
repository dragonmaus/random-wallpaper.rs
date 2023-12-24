use getopt::Opt;
use rand::seq::IteratorRandom;

mod util;

program::main!("random-wallpaper");

fn usage_line(program_name: &str) -> String {
    format!("Usage: {} [DIR...]", program_name)
}

fn print_usage(program_name: &str) {
    println!("{}", usage_line(program_name));
    println!("  -q   do not display results");
    println!();
    println!("  -h   display this help");
}

fn program(name: &str) -> program::Result {
    let mut args = program::args();
    let mut opts = getopt::Parser::new(&args, "hq");

    let mut quiet = false;
    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('q', None) => quiet = true,
                Opt('h', None) => {
                    print_usage(name);
                    return Ok(0);
                },
                _ => unreachable!(),
            },
        }
    }

    let mut args = args.split_off(opts.index());

    // Default to current directory if none specified
    if args.is_empty() {
        args.push(String::from("."));
    }

    let current = util::wallpaper::get()?;
    let files = util::scan_files(args)?;

    if files.is_empty() {
        return Err(Box::new(std::io::Error::other("no image files found")));
    }

    let mut rng = rand::thread_rng();

    // Select a random file that isn't the currently set wallpaper
    match files.into_iter().filter(|p| *p != current).choose(&mut rng) {
        Some(path) => {
            if !quiet {
                println!("{}", path.to_string_lossy());
            };
            util::wallpaper::set(&path)?
        },
        None => return Err(Box::new(std::io::Error::other("unexpected error"))),
    };

    Ok(0)
}
