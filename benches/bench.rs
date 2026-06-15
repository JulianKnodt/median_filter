#![feature(test)]

extern crate test;

use image;

const IMG: &'static [u8] = include_bytes!("../valve.png");

#[bench]
fn bench_sobel_vertical(b: &mut test::Bencher) {
    let img = image::load_from_memory(IMG).unwrap();
    let w = img.width();
    let h = img.height();

    let mut img = img.into_luma8().into_raw();

    let mut buf = (0..w * h).map(|_| 0u8).collect::<Vec<_>>();

    use median_filter::median_filter_3x3;
    b.iter(|| median_filter_3x3(&img, &mut buf, w as usize, h as usize));
}

/*
#[bench]
fn bench_sobel_horizontal(b: &mut test::Bencher) {
    let img = image::load_from_memory(IMG).unwrap();
    let w = img.width();
    let h = img.height();

    let mut img = img.into_luma8().into_raw();

    let mut buf = (0..w * h).map(|_| 0i8).collect::<Vec<_>>();

    use median_filter::{median_filter_5x5};
    b.iter(|| sobel_u8::sobel_horizontal(&mut img, w as usize, h as usize, &mut buf));
}
*/
