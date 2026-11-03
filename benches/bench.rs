#![feature(test)]

extern crate test;

use image;

const IMG: &'static [u8] = include_bytes!("../valve.png");

#[bench]
fn bench_3x3(b: &mut test::Bencher) {
    let img = image::load_from_memory(IMG).unwrap();
    let w = img.width();
    let h = img.height();

    let img = img.into_luma8().into_raw();

    let mut buf = (0..w * h).map(|_| 0u8).collect::<Vec<_>>();

    use median_filter::median_filter_3x3;
    b.iter(|| median_filter_3x3(&img, &mut buf, w as usize, h as usize));
}

#[bench]
fn bench_5x5(b: &mut test::Bencher) {
    let img = image::load_from_memory(IMG).unwrap();
    let w = img.width();
    let h = img.height();

    let img = img.into_luma8().into_raw();

    let mut buf = (0..w * h).map(|_| 0u8).collect::<Vec<_>>();

    use median_filter::median_filter_5x5;
    b.iter(|| median_filter_5x5(&img, &mut buf, w as usize, h as usize));
}

#[bench]
fn bench_circ_diam7(b: &mut test::Bencher) {
    let img = image::load_from_memory(IMG).unwrap();
    let w = img.width();
    let h = img.height();

    let img = img.into_luma8().into_raw();

    let mut buf = (0..w * h).map(|_| 0u8).collect::<Vec<_>>();

    use median_filter::circ_diam7;
    b.iter(|| circ_diam7(&img, &mut buf, w as usize, h as usize));
}

#[bench]
fn bench_circ_diam9(b: &mut test::Bencher) {
    let img = image::load_from_memory(IMG).unwrap();
    let w = img.width();
    let h = img.height();

    let img = img.into_luma8().into_raw();

    let mut buf = (0..w * h).map(|_| 0u8).collect::<Vec<_>>();

    use median_filter::circ_diam9;
    b.iter(|| circ_diam9(&img, &mut buf, w as usize, h as usize));
}

#[bench]
fn bench_circ_diam10(b: &mut test::Bencher) {
    let img = image::load_from_memory(IMG).unwrap();
    let w = img.width();
    let h = img.height();

    let img = img.into_luma8().into_raw();

    let mut buf = (0..w * h).map(|_| 0u8).collect::<Vec<_>>();

    use median_filter::circ_diam10;
    b.iter(|| circ_diam10(&img, &mut buf, w as usize, h as usize));
}
