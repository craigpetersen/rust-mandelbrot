mod utils;
mod complex;

use utils::write_image;
use complex:: {
    parse_pair,
    parse_complex,
    render_async
};
use std::time::{Instant};


fn main() {
    use std::env;
    let cpus = num_cpus::get();
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
        .expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4])
        .expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let start = Instant::now();

    render_async(cpus, &mut pixels, bounds, upper_left, lower_right);

    let duration = start.elapsed();

    write_image(&args[1], &pixels, bounds)
        .expect("error writing file");

    println!("Done! Render time on {} cpus ~ {} ms", cpus, duration.as_millis().to_string());
}
