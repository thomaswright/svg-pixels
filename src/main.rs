use clap::Parser;
use image::Rgba;
use std::fs::File;
use std::io::{self, Write};

#[derive(Parser)]
struct Cli {
    input: String,
    output: String,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let img = image::open(&args.input)
        .expect("Failed to open image")
        .to_rgba8();
    let (width, height) = img.dimensions();
    let mut file = File::create(&args.output)?;

    writeln!(
        file,
        r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="{}" height="{}">"#,
        width, height
    )?;

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let Rgba([r, g, b, a]) = pixel;
            // Skip fully transparent pixels (alpha = 0)
            if *a == 0 {
                continue;
            }

            writeln!(
                file,
                r#"<rect x="{}" y="{}" width="1" height="1" fill="rgba({}, {}, {}, {:.2})"/>"#,
                x,
                y,
                r,
                g,
                b,
                *a as f32 / 255.0
            )?;
        }
    }

    writeln!(file, "</svg>")?;

    println!("SVG file '{}' created successfully!", args.output);
    Ok(())
}
