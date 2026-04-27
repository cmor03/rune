//! Output backends for rendered frames.

use std::fs::{create_dir_all, read_to_string, File, OpenOptions};
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

/// Orientation transform applied while writing to the framebuffer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Transform {
    /// Write the canvas as-is.
    None,
    /// Rotate the portrait canvas clockwise into a landscape framebuffer.
    Rotate90,
    /// Rotate the portrait canvas counter-clockwise into a landscape framebuffer.
    Rotate270,
}

impl Transform {
    /// Parses a transform name.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "0" | "none" => Some(Self::None),
            "90" | "rotate90" => Some(Self::Rotate90),
            "270" | "rotate270" => Some(Self::Rotate270),
            _ => None,
        }
    }
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

    fn bytes_per_pixel(self) -> usize {
        match self {
            Self::Rgb565 => 2,
            Self::Xrgb8888 => 4,
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
    transform: Transform,
    width: usize,
    height: usize,
    stride: usize,
}

impl FbdevSink {
    /// Opens a framebuffer device such as `/dev/fb0`.
    pub fn open(path: impl AsRef<Path>, format: PixelFormat) -> io::Result<Self> {
        Self::open_with_transform(path, format, Transform::None)
    }

    /// Opens a framebuffer device with an output transform.
    pub fn open_with_transform(
        path: impl AsRef<Path>,
        format: PixelFormat,
        transform: Transform,
    ) -> io::Result<Self> {
        Self::open_configured(path, format, transform, None, None, None)
    }

    /// Opens a framebuffer device with optional geometry overrides.
    pub fn open_configured(
        path: impl AsRef<Path>,
        format: PixelFormat,
        transform: Transform,
        width: Option<usize>,
        height: Option<usize>,
        stride: Option<usize>,
    ) -> io::Result<Self> {
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
        let fb_name = path.file_name().and_then(|name| name.to_str());
        let detected = fb_name.and_then(detect_geometry);
        let default_width = if matches!(transform, Transform::None) {
            WIDTH
        } else {
            HEIGHT
        };
        let default_height = if matches!(transform, Transform::None) {
            HEIGHT
        } else {
            WIDTH
        };
        let width = width.or_else(|| detected.map(|geo| geo.0)).unwrap_or(default_width);
        let height = height
            .or_else(|| detected.map(|geo| geo.1))
            .unwrap_or(default_height);
        let stride = stride
            .or_else(|| fb_name.and_then(detect_stride))
            .unwrap_or(width * format.bytes_per_pixel());
        Ok(Self {
            file,
            format,
            transform,
            width,
            height,
            stride,
        })
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
        let transformed = transformed_pixels(canvas, self.transform);
        let source_width = if matches!(self.transform, Transform::None) {
            WIDTH
        } else {
            HEIGHT
        };
        let source_height = if matches!(self.transform, Transform::None) {
            HEIGHT
        } else {
            WIDTH
        };
        let out = pack_fb(
            &transformed,
            source_width,
            source_height,
            self.width,
            self.height,
            self.stride,
            self.format,
        );
        self.file.write_all(&out)
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

fn pack_fb(
    source: &[u8],
    source_width: usize,
    source_height: usize,
    fb_width: usize,
    fb_height: usize,
    stride: usize,
    format: PixelFormat,
) -> Vec<u8> {
    let bytes_per_pixel = format.bytes_per_pixel();
    let mut out = vec![0; stride * fb_height];
    let copy_width = source_width.min(fb_width);
    let copy_height = source_height.min(fb_height);
    for y in 0..copy_height {
        for x in 0..copy_width {
            let px = source[y * source_width + x];
            let dst = y * stride + x * bytes_per_pixel;
            match format {
                PixelFormat::Rgb565 => {
                    let v = px as u16;
                    let r = (v >> 3) & 0x1f;
                    let g = (v >> 2) & 0x3f;
                    let b = (v >> 3) & 0x1f;
                    let rgb = (r << 11) | (g << 5) | b;
                    out[dst..dst + 2].copy_from_slice(&rgb.to_le_bytes());
                }
                PixelFormat::Xrgb8888 => {
                    out[dst..dst + 4].copy_from_slice(&[px, px, px, 0x00]);
                }
            }
        }
    }
    out
}

fn transformed_pixels(canvas: &Canvas, transform: Transform) -> Vec<u8> {
    match transform {
        Transform::None => canvas.pixels().to_vec(),
        Transform::Rotate90 => {
            let mut out = Vec::with_capacity(WIDTH * HEIGHT);
            for dy in 0..WIDTH {
                for dx in 0..HEIGHT {
                    let sx = dy;
                    let sy = HEIGHT - 1 - dx;
                    out.push(canvas.pixels()[sy * WIDTH + sx]);
                }
            }
            out
        }
        Transform::Rotate270 => {
            let mut out = Vec::with_capacity(WIDTH * HEIGHT);
            for dy in 0..WIDTH {
                for dx in 0..HEIGHT {
                    let sx = WIDTH - 1 - dy;
                    let sy = dx;
                    out.push(canvas.pixels()[sy * WIDTH + sx]);
                }
            }
            out
        }
    }
}

fn detect_geometry(fb_name: &str) -> Option<(usize, usize)> {
    let path = format!("/sys/class/graphics/{fb_name}/virtual_size");
    let text = read_to_string(path).ok()?;
    parse_pair(&text)
}

fn detect_stride(fb_name: &str) -> Option<usize> {
    let path = format!("/sys/class/graphics/{fb_name}/stride");
    read_to_string(path).ok()?.trim().parse().ok()
}

fn parse_pair(text: &str) -> Option<(usize, usize)> {
    let mut parts = text.trim().split(',');
    let width = parts.next()?.parse().ok()?;
    let height = parts.next()?.parse().ok()?;
    Some((width, height))
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
