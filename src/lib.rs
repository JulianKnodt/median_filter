use rayon::prelude::*;

//pub mod sorting_networks;

mod counted_bitvec;
pub mod wavelet_matrix;

pub mod weighted;

macro_rules! sorting_step {
  ($arr: ident, $( $check: expr ),+) => {{$(
    let [v0, v1] = $check.map(|c| unsafe { *$arr.get_unchecked(c) });
    if v1 < v0 {
      unsafe { *$arr.get_unchecked_mut($check[0]) = v1 };
      unsafe { *$arr.get_unchecked_mut($check[1]) = v0 };
    }
  )+}}
}

pub trait Value: Copy + PartialEq + PartialOrd + Send + Sync {}

impl Value for u8 {}
impl Value for u16 {}
impl Value for u32 {}
impl Value for f32 {}
impl Value for i8 {}
impl Value for i16 {}

/// 3x3 single channel median filter using sorting networks
pub fn median_filter_3x3<T: Value>(img: &[T], out: &mut [T], w: usize, h: usize) {
    assert_eq!(img.len(), w * h);
    assert!(out.len() >= img.len());

    /*
    for y in 0..h {
        let prev_y = y.saturating_sub(1);
        let next_y = (y + 1).min(h - 1);
        for x in 0..w {
            let prev_x = x.saturating_sub(1);
            let next_x = (x + 1).min(w - 1);

            let mut arr = [
                [prev_y, prev_x],
                [prev_y, x],
                [prev_y, next_x],
                //
                [y, prev_x],
                [y, x],
                [y, next_x],
                //
                [next_y, prev_x],
                [next_y, x],
                [next_y, next_x],
            ]
            .map(|[y, x]| unsafe { *img.get_unchecked(y * w + x) });

            // sorting network
            sorting_step!(arr, [0, 3], [1, 7], [2, 5], [4, 8]);
            sorting_step!(arr, [0, 7], [2, 4], [3, 8], [5, 6]);
            sorting_step!(arr, [0, 2], [1, 3], [4, 5], [7, 8]);
            sorting_step!(arr, [1, 4], [3, 6], [5, 7]);
            sorting_step!(arr, [0, 1], [2, 4], [3, 5], [6, 8]);
            sorting_step!(arr, [2, 3], [4, 5], [6, 7]);
            sorting_step!(arr, [1, 2], [3, 4], [5, 6]);

            debug_assert!(arr.is_sorted());

            *unsafe { out.get_unchecked_mut(y * w + x) } = arr[4];
        }
    }
    */
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let prev_y = y.saturating_sub(1);
        let next_y = (y + 1).min(h - 1);
        for (x, dst) in row.iter_mut().enumerate() {
            let prev_x = x.saturating_sub(1);
            let next_x = (x + 1).min(w - 1);

            let mut arr = [
                [prev_y, prev_x],
                [prev_y, x],
                [prev_y, next_x],
                //
                [y, prev_x],
                [y, x],
                [y, next_x],
                //
                [next_y, prev_x],
                [next_y, x],
                [next_y, next_x],
            ]
            .map(|[y, x]| unsafe { *img.get_unchecked(y * w + x) });

            // sorting network
            sorting_step!(arr, [0, 3], [1, 7], [2, 5], [4, 8]);
            sorting_step!(arr, [0, 7], [2, 4], [3, 8], [5, 6]);
            sorting_step!(arr, [0, 2], [1, 3], [4, 5], [7, 8]);
            sorting_step!(arr, [1, 4], [3, 6], [5, 7]);
            sorting_step!(arr, [0, 1], [2, 4], [3, 5], [6, 8]);
            sorting_step!(arr, [2, 3], [4, 5], [6, 7]);
            sorting_step!(arr, [1, 2], [3, 4], [5, 6]);
            //sorting_step!(arr, [3, 4]);

            *dst = arr[4];
        }
    });
}

/// 5x5 single channel median filter using sorting networks
pub fn median_filter_5x5<T: Value>(img: &[T], out: &mut [T], w: usize, h: usize) {
    assert_eq!(img.len(), w * h);
    assert!(out.len() >= img.len());

    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let m1_y = y.saturating_sub(1);
        let m2_y = y.saturating_sub(2);
        let p1_y = (y + 1).min(h - 1);
        let p2_y = (y + 2).min(h - 1);
        for (x, dst) in row.iter_mut().enumerate() {
            let m2_x = x.saturating_sub(2);
            let m1_x = x.saturating_sub(1);

            let p1_x = (x + 1).min(w - 1);
            let p2_x = (x + 2).min(w - 1);

            let mut arr = [
                [p2_y, p2_x],
                [p2_y, p1_x],
                [p2_y, x],
                [p2_y, m1_x],
                [p2_y, m2_x],
                //
                [p1_y, p2_x],
                [p1_y, p1_x],
                [p1_y, x],
                [p1_y, m1_x],
                [p1_y, m2_x],
                //
                [y, p2_x],
                [y, p1_x],
                [y, x],
                [y, m1_x],
                [y, m2_x],
                //
                [m1_y, p2_x],
                [m1_y, p1_x],
                [m1_y, x],
                [m1_y, m1_x],
                [m1_y, m2_x],
                //
                [m2_y, p2_x],
                [m2_y, p1_x],
                [m2_y, x],
                [m2_y, m1_x],
                [m2_y, m2_x],
            ]
            .map(|[y, x]| unsafe { *img.get_unchecked(y * w + x) });

            // sorting network
            sorting_step!(
                arr,
                [0, 1],
                [2, 3],
                [4, 5],
                [6, 7],
                [8, 9],
                [10, 11],
                [12, 13],
                [14, 15],
                [16, 17],
                [18, 19],
                [20, 21],
                [22, 23]
            );

            sorting_step!(
                arr,
                [0, 2],
                [1, 3],
                [4, 6],
                [5, 7],
                [8, 10],
                [9, 11],
                [12, 14],
                [13, 15],
                [16, 18],
                [17, 19],
                [20, 22],
                [21, 24]
            );
            sorting_step!(
                arr,
                [0, 4],
                [1, 5],
                [2, 6],
                [3, 7],
                [8, 12],
                [9, 13],
                [10, 14],
                [11, 15],
                [16, 20],
                [21, 22],
                [23, 24]
            );
            sorting_step!(
                arr,
                [0, 8],
                [1, 12],
                [2, 10],
                [3, 14],
                [4, 9],
                [5, 13],
                [6, 11],
                [7, 15],
                [17, 22],
                [18, 21],
                [19, 24]
            );
            sorting_step!(
                arr,
                [1, 18],
                [3, 9],
                [5, 17],
                [6, 20],
                [7, 13],
                [11, 14],
                [12, 22],
                [15, 24],
                [21, 23]
            );
            sorting_step!(
                arr,
                [1, 16],
                [3, 12],
                [5, 21],
                [6, 18],
                [7, 11],
                [10, 17],
                [14, 23],
                [19, 20]
            );
            sorting_step!(
                arr,
                [0, 1],
                [2, 5],
                [4, 16],
                [6, 8],
                [7, 18],
                [9, 21],
                [10, 14],
                [11, 13],
                [12, 19],
                [15, 23],
                [20, 22]
            );
            sorting_step!(
                arr,
                [1, 2],
                [3, 5],
                [4, 6],
                [7, 9],
                [8, 12],
                [10, 16],
                [11, 20],
                [13, 22],
                [14, 17],
                [15, 18],
                [19, 21]
            );
            sorting_step!(
                arr,
                [1, 4],
                [2, 6],
                [3, 7],
                [5, 9],
                [8, 10],
                [11, 14],
                [12, 16],
                [13, 17],
                [15, 19],
                [18, 20],
                [22, 23]
            );
            sorting_step!(
                arr,
                [2, 4],
                [3, 8],
                [5, 10],
                [7, 12],
                [9, 16],
                [11, 15],
                [13, 19],
                [14, 21],
                [17, 18],
                [20, 22]
            );
            sorting_step!(
                arr,
                [3, 4],
                [5, 8],
                [6, 7],
                [9, 12],
                [10, 11],
                [13, 16],
                [14, 15],
                [17, 19],
                [18, 21]
            );
            sorting_step!(
                arr,
                [5, 6],
                [7, 8],
                [9, 10],
                [11, 12],
                [13, 14],
                [15, 16],
                [17, 18],
                [20, 21]
            );
            sorting_step!(
                arr,
                [4, 5],
                [6, 7],
                [8, 9],
                [10, 11],
                [12, 13],
                [14, 15],
                [16, 17],
                [18, 19]
            );

            debug_assert!(arr.is_sorted());
            *dst = arr[12];
        }
    });
}

/// Single channel median filter using an explicit sort of the following circle
/// with a diameter of 7 pixels
/**
  0 1 2 3 4 5 6
0 - - x x x - -
1 - x       x -
2 x           x
3 x           x
4 x           x
5 - x       x -
6 - - x x x - -
**/
pub fn circ_diam7<T: Value>(img: &[T], out: &mut [T], w: usize, h: usize) {
    assert_eq!(img.len(), w * h);
    assert!(out.len() >= img.len());

    let ss = usize::saturating_sub;
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (x, dst) in row.iter_mut().enumerate() {
            #[rustfmt::skip]
            let arr: [_; 37] = [
                          [2,0],[3,0],[4,0],
                    [1,1],[2,1],[3,1],[4,1],[5,1],
              [0,2],[1,2],[2,2],[3,2],[4,2],[5,2],[6,2],
              [0,3],[1,3],[2,3],[3,3],[4,3],[5,3],[6,3],
              [0,4],[1,4],[2,4],[3,4],[4,4],[5,4],[6,4],
                    [1,5],[2,5],[3,5],[4,5],[5,5],
                          [2,6],[3,6],[4,6],
            ];

            let mut arr = arr.map(|[dy, dx]| {
                let ny = ss(y + dy, 3).min(h - 1);
                let nx = ss(x + dx, 3).min(w - 1);
                unsafe { *img.get_unchecked(ny * w + nx) }
            });
            use std::cmp::Ordering::*;
            let (_, &mut mid, _) =
                arr.select_nth_unstable_by(37 / 2, |a, b| if a < b { Less } else { Greater });
            *dst = mid;
        }
    });
}

/// Single channel median filter using an explicit sort of the following circle
/// with a diameter of 9 pixels
/**
  0 1 2 3 4 5 6 7 8
0 - - - x x x - - -
1 - x x       x x -
2 - x           x -
3 x               x
4 x               x
5 x               x
6 - x           x -
7 - x x       x x -
8 - - - x x x - - -
**/
pub fn circ_diam9<T: Value>(img: &[T], out: &mut [T], w: usize, h: usize) {
    assert_eq!(img.len(), w * h);
    assert!(out.len() >= img.len());

    let ss = usize::saturating_sub;
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (x, dst) in row.iter_mut().enumerate() {
            #[rustfmt::skip]
            let arr: [_; 61] = [
                                [3,0],[4,0],[5,0],
                    [1,1],[2,1],[3,1],[4,1],[5,1],[6,1],[7,1],
                    [1,2],[2,2],[3,2],[4,2],[5,2],[6,2],[7,2],
              [0,3],[1,3],[2,3],[3,3],[4,3],[5,3],[6,3],[7,3],[8,3],
              [0,4],[1,4],[2,4],[3,4],[4,4],[5,4],[6,4],[7,4],[8,4],
              [0,5],[1,5],[2,5],[3,5],[4,5],[5,5],[6,5],[7,5],[8,5],
                    [1,6],[2,6],[3,6],[4,6],[5,6],[6,6],[7,6],
                    [1,7],[2,7],[3,7],[4,7],[5,7],[6,7],[7,7],
                                [3,8],[4,8],[5,8],
            ];

            let mut arr = arr.map(|[dy, dx]| {
                let ny = ss(y + dy, 4).min(h - 1);
                let nx = ss(x + dx, 4).min(w - 1);
                unsafe { *img.get_unchecked(ny * w + nx) }
            });
            use std::cmp::Ordering::*;
            let (_, &mut mid, _) =
                arr.select_nth_unstable_by(61 / 2, |a, b| if a < b { Less } else { Greater });
            *dst = mid;
        }
    });
}

/// Single channel median filter using an explicit sort of the following circle
/// with a diameter of 10 pixels
/**
  0 1 2 3 4 5 6 7 8 9 0
0 - - - - x x x - - - -
1 - - x x       x x - -
2 - x               x -
3 - x               x -
4 x                   x
5 x                   x
6 x                   x
7 - x               x -
8 - x               x -
9 - - x x       x x - -
0 - - - - x x x - - - -
**/
pub fn circ_diam10<T: Value>(img: &[T], out: &mut [T], w: usize, h: usize) {
    assert_eq!(img.len(), w * h);
    assert!(out.len() >= img.len());

    let ss = usize::saturating_sub;
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (x, dst) in row.iter_mut().enumerate() {
            #[rustfmt::skip]
            let arr: [_; 89] = [
                                      [4,0],[5,0],[6,0],
                          [2,1],[3,1],[4,1],[5,1],[6,1],[7,1],[8,1],
                    [1,2],[2,2],[3,2],[4,2],[5,2],[6,2],[7,2],[8,2],[9,2],
                    [1,3],[2,3],[3,3],[4,3],[5,3],[6,3],[7,3],[8,3],[9,3],
              [0,4],[1,4],[2,4],[3,4],[4,4],[5,4],[6,4],[7,4],[8,4],[9,4],[10,4],
              [0,5],[1,5],[2,5],[3,5],[4,5],[5,5],[6,5],[7,5],[8,5],[9,5],[10,5],
              [0,6],[1,6],[2,6],[3,6],[4,6],[5,6],[6,6],[7,6],[8,6],[9,6],[10,6],
                    [1,7],[2,7],[3,7],[4,7],[5,7],[6,7],[7,7],[8,7],[9,7],
                    [1,8],[2,8],[3,8],[4,8],[5,8],[6,8],[7,8],[8,8],[9,8],
                          [2,9],[3,9],[4,9],[5,9],[6,9],[7,9],[8,9],
                                    [4,10],[5,10],[6,10],
            ];

            let mut arr = arr.map(|[dy, dx]| {
                let ny = ss(y + dy, 5).min(h - 1);
                let nx = ss(x + dx, 5).min(w - 1);
                unsafe { *img.get_unchecked(ny * w + nx) }
            });
            use std::cmp::Ordering::*;
            let (_, &mut mid, _) =
                arr.select_nth_unstable_by(89 / 2, |a, b| if a < b { Less } else { Greater });
            *dst = mid;
        }
    });
}

#[test]
fn test_median_filter_3x3() {
    let mut img = vec![];
    const N: usize = 25;
    for i in 0..N {
        img.push((i as u8).wrapping_mul(16));
    }
    let mut out = vec![0; N];

    median_filter_3x3(&img, &mut out, 5, 5);
}

#[test]
fn test_median_filter_5x5() {
    let mut img = vec![];
    const N: usize = 25;
    for i in 0..N {
        img.push((i as u8).wrapping_mul(16));
    }
    let mut out = vec![0; N];

    median_filter_5x5(&img, &mut out, 5, 5);
}

#[test]
fn test_median_filter_3x3_u16() {
    let mut img = vec![];
    const N: usize = 25;
    for i in 0..N {
        img.push((i as u16).wrapping_mul(16));
    }
    let mut out = vec![0; N];

    median_filter_3x3(&img, &mut out, 5, 5);
}

#[test]
fn test_median_filter_5x5_u16() {
    let mut img = vec![];
    const N: usize = 25;
    for i in 0..N {
        img.push((i as u16).wrapping_mul(16));
    }
    let mut out = vec![0; N];

    median_filter_5x5(&img, &mut out, 5, 5);
}
