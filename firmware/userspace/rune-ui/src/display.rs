//! Output backends for rendered frames.

use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{self, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::canvas::{Canvas, HEIGHT, WIDTH};

/// A destination for rendered UI frames.
pub trait FrameSink {
    /// Writes one frame.
    fn write_frame(&mut self, screen_name: &str, frame_index: u32, canvas: &Canvas)
        -> io::Result<()>;
}

/// Pixel formats supported by the simple Linux framebuffer sink.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PixelFormat {
    /// 16-bit little-endian RGB565.
    Rgb565,
    /// 32-bit little-endian XRGB8888.
    Xrgb8888,
}

impl PixelFormat {
    /// Parses a pixel format name.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "rgb565" => Some(Self::Rgb565),
            "xrgb8888" | "xrgb" => Some(Self::Xrgb8888),
            _ => None,
        }
    }
}

/// A simple sink that writes one frame directly to a Linux framebuffer device.
///
/// This intentionally does not perform mode detection. It is for early Pi LCD
/// testing where the screen is already configured by the kernel overlay.
pub struct FbdevSink {
    file: File,
    format: PixelFormat,
}

impl FbdevSink {
    /// Opens a framebuffer device such as `/dev/fb0`.
    pub fn open(path: impl AsRef<Path>, format: PixelFormat) -> io::Result<Self> {
        let path = path.as_ref();
        let file = OpenOptions::new().write(true).open(path).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!(
                    "could not open framebuffer device '{}': {err}. \
Check `ls -l /dev/fb*` on the Pi; your LCD may be exposed as a different fbdev, as DRM, or not registered by a kernel overlay yet.",
                    path.display()
                ),
            )
        })?;
        Ok(Self { file, format })
    }
}

impl FrameSink for FbdevSink {
    fn write_frame(
        &mut self,
        _screen_name: &str,
        _frame_index: u32,
        canvas: &Canvas,
    ) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        match self.format {
            PixelFormat::Rgb565 => {
                let mut out = Vec::with_capacity(WIDTH * HEIGHT * 2);
                for px in canvas.pixels() {
                    let v = *px as u16;
                    let r = (v >> 3) & 0x1f;
                    let g = (v >> 2) & 0x3f;
                    let b = (v >> 3) & 0x1f;
                    let rgb = (r << 11) | (g << 5) | b;
                    out.extend_from_slice(&rgb.to_le_bytes());
                }
                self.file.write_all(&out)
            }
            PixelFormat::Xrgb8888 => {
                let mut out = Vec::with_capacity(WIDTH * HEIGHT * 4);
                for px in canvas.pixels() {
                    out.extend_from_slice(&[*px, *px, *px, 0x00]);
                }
                self.file.write_all(&out)
            }
        }
    }
}

/// A sink that writes binary PGM images.
pub struct PgmSink {
    dir: PathBuf,
}

impl PgmSink {
    /// Creates a PGM sink rooted at `dir`.
    pub fn new(dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref().to_path_buf();
        create_dir_all(&dir)?;
        Ok(Self { dir })
    }
}

impl FrameSink for PgmSink {
    fn write_frame(
        &mut self,
        screen_name: &str,
        frame_index: u32,
        canvas: &Canvas,
    ) -> io::Result<()> {
        let path = self
            .dir
            .join(format!("{screen_name}-{frame_index:03}.pgm"));
        let mut file = File::create(path)?;
        write!(file, "P5\n{} {}\n255\n", WIDTH, HEIGHT)?;
        file.write_all(canvas.pixels())
    }
}

/// Converts a grayscale canvas into 1-bit packed rows for e-ink experiments.
pub fn pack_1bpp(canvas: &Canvas) -> Vec<u8> {
    let bytes_per_row = (WIDTH + 7) / 8;
    let mut out = vec![0xff; bytes_per_row * HEIGHT];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = canvas.pixels()[y * WIDTH + x];
            if pixel < 128 {
                out[y * bytes_per_row + x / 8] &= !(1 << (7 - (x % 8)));
            }
        }
    }
    out
}
