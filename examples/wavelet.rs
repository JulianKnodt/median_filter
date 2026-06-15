use median_filter::wavelet_matrix::MedianWavelet;

pub fn main() -> std::io::Result<()> {
    let args = parse_args::parse_args!(
      "Run a median filter on the input mesh",
      Input("-i", "--input"; "Input Image") => input : String = String::new(),
      Output("-o", "--output"; "Output image") => output : String = String::new(),
      Rad5("-r", "--radius"; "Radius of median filter") => rad : u32 = 5,
    );

    let Ok(img) = image::open(&args.input) else {
        eprintln!("Failed to open {}", args.input);
        return Ok(());
    };

    let img = img.to_luma8();
    let w = img.width();
    let h = img.height();
    let mut out = img.clone();
    out.fill(0);

    let mw = MedianWavelet::new(&img, h, w);
    mw.median_with_cut_border(args.rad, &mut out);

    let Ok(()) = out.save(&args.output) else {
        eprintln!("Failed to save {}", args.output);
        return Ok(());
    };

    Ok(())
}
