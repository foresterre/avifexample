use image::GenericImageView;

fn main() {
    let args = std::env::args().take(3).collect::<Vec<_>>();

    println!("Usage: avifexample [optional: input file (default: wh1616.png)] [optional: output file (default: wh1616.avif)]");

    let dynamic_image = if args.len() >= 2 {
        image::open(&args[1])
    } else {
        image::open("wh1616.png")
    }.expect("Unable to open image!");

    let data = dynamic_image.to_bytes();
    let (width, height) = dynamic_image.dimensions();
    let color_type = dynamic_image.color();

    let out_file = if args.len() >= 3 {
        std::fs::File::create(&args[2]).expect(&format!("Unable to open file '{}'!", &args[2]))
    } else {
        std::fs::File::create("wh1616.avif").expect("Unable to open file 'wh1616.avif'")
    };

    let avif_encoder = image::avif::AvifEncoder::new(out_file);
    avif_encoder
        .write_image(&data, width, height, color_type)
        .expect("Unable to write image");

    println!("Completed conversion to AVIF!");
}
