use mandelbrot;

#[cfg(test)]
pub fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10,20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", ','), Some((0.5,1.5)));
}

pub fn test_parse_complex() {
    assert_eq!(test_parse_complex("1.25, -0.0625"));
    assert_eq!(test_parse_complex({ re: 1.25, im: -0.0625 } ));
    assert_eq!(test_parse_complex(",-0.0625"), None);
}