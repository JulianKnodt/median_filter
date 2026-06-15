use median_filter::{median_filter_3x3, median_filter_5x5};

pub fn main() -> std::io::Result<()> {
    let args = parse_args::parse_args!(
      "Run a median filter on the input mesh",
      Input("-i", "--input"; "Input Image") => input : String = String::new(),
      Output("-o", "--output"; "Output image") => output : String = String::new(),
      Rad5("-r5", "--radius-5"; "Run with radius of 5") => r5 : bool = false => true,
    );

    let Ok(img) = image::open(&args.input) else {
        eprintln!("Failed to open {}", args.input);
        return Ok(());
    };

    let img = img.to_luma8();
    let w = img.width() as usize;
    let h = img.height() as usize;
    let mut out = img.clone();

    if args.r5 {
        median_filter_5x5(&img, &mut out, w, h);
    } else {
        median_filter_3x3(&img, &mut out, w, h);
    }

    let Ok(()) = out.save(&args.output) else {
        eprintln!("Failed to save {}", args.output);
        return Ok(());
    };

    Ok(())
}
