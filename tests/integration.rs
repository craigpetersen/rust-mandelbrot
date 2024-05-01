use utils;

#[test]
fn it_writes_image() {

    let filename = "test.png";
    let pixels = [255,255,255];
    let bounds = (2,2);
    let mut image = write_image(filename, pixels, bounds);

    assert_eq!(image, Ok(()))
}