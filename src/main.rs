mod utils;
mod complex;

use utils::write_image;
use complex:: {
    parse_pair,
    parse_complex,
    render_async
};


fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!("Usage: {} FILE THREADS PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 64 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }
    let arg = match args.get(2) {
        Some(val) => val,
        None => {
            println!("invalid argument supplied!");
            return;
        }
    };

    let threads = match arg.parse::<usize>(){
        Ok(val) => val,
        Err(e) => {
            eprintln!("unable to parse number of threads from argument {}", e);
            return;
        }
    };

    let bounds = parse_pair(&args[3], 'x')
        .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[4])
        .expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[5])
        .expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render_async(threads, &mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds)
        .expect("error writing file");

    println!("Done!");
}
