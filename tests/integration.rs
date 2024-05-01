#[path="../src/utils.rs"]
mod utils;

use utils::write_image;
#[test]
fn it_writes_image() {
    let filename = "test.png";
    let bounds: (usize, usize) = (10, 10);
    let pixels: Vec<u8> = vec![0, 100];
    let e = write_image(&filename, &pixels, bounds);
    
    assert_eq!(e.is_err(), true);
}