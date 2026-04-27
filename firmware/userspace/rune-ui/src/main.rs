//! Command-line renderer for Rune UI mock frames.

use std::env;
use std::io;

use rune_ui::animation::FrameContext;
use rune_ui::display::{FbdevSink, FrameSink, PgmSink, PixelFormat};
use rune_ui::{render, Screen};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let screen = value_after(&args, "--screen")
        .and_then(Screen::parse)
        .unwrap_or(Screen::Home);
    let out_dir = value_after(&args, "--out").unwrap_or("target/ui-frames");
    let frames = value_after(&args, "--frames")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(1)
        .max(1);
    let backend = value_after(&args, "--backend").unwrap_or("pgm");

    if backend == "fbdev" {
        let fb = value_after(&args, "--fb").unwrap_or("/dev/fb1");
        let format = value_after(&args, "--format")
            .and_then(PixelFormat::parse)
            .unwrap_or(PixelFormat::Rgb565);
        let mut sink = FbdevSink::open(fb, format)?;
        for frame in 0..frames {
            let ctx = FrameContext::new(frame, frames);
            let canvas = render(screen, ctx);
            sink.write_frame(screen.name(), frame, &canvas)?;
        }
        eprintln!(
            "rendered {frames} frame(s) for '{}' to {fb}",
            screen.name()
        );
    } else {
        let mut sink = PgmSink::new(out_dir)?;
        for frame in 0..frames {
            let ctx = FrameContext::new(frame, frames);
            let canvas = render(screen, ctx);
            sink.write_frame(screen.name(), frame, &canvas)?;
        }
        eprintln!(
            "rendered {frames} frame(s) for '{}' into {out_dir}",
            screen.name()
        );
    }
    Ok(())
}

fn value_after<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|pair| pair.first().map(String::as_str) == Some(key))
        .and_then(|pair| pair.get(1))
        .map(String::as_str)
}
