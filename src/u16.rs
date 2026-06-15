macro_rules! sorting_step {
  ($arr: ident, $( $check: expr ),+) => {{$(
    let [v0, v1] = $check.map(|c| $arr[c]);
    if v1 < v0 {
      unsafe { *$arr.get_unchecked_mut($check[0]) = v1 };
      unsafe { *$arr.get_unchecked_mut($check[1]) = v0 };
    }
  )+}}
}

/// 3x3 single channel median filter using sorting networks
pub fn median_filter_3x3(img: &[u16], out: &mut [u16], w: usize, h: usize) {
    assert_eq!(img.len(), w * h);
    assert!(out.len() >= img.len());

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

            out[y * w + x] = arr[4];
        }
    }
}

/// 3x3 single channel median filter using sorting networks
pub fn median_filter_5x5(img: &[u16], out: &mut [u16], w: usize, h: usize) {
    assert_eq!(img.len(), w * h);
    assert!(out.len() >= img.len());

    for y in 0..h {
        let m1_y = y.saturating_sub(1);
        let m2_y = y.saturating_sub(2);
        let p1_y = (y + 1).min(h - 1);
        let p2_y = (y + 2).min(h - 1);
        for x in 0..w {
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
            out[y * w + x] = arr[12];
        }
    }
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
