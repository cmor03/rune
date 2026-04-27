//! Command-line renderer for Rune UI mock frames.

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;
use std::time::Duration;

use rune_ui::animation::FrameContext;
use rune_ui::app::{render_state, Command, UiState};
use rune_ui::display::{FbdevSink, FrameSink, PgmSink, PixelFormat, Transform};
use rune_ui::{render, Screen};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let socket = value_after(&args, "--socket").unwrap_or("/tmp/rune-ui.sock");

    if let Some(command) = value_after(&args, "--send") {
        return send_command(socket, command);
    }

    let screen = value_after(&args, "--screen")
        .and_then(Screen::parse)
        .unwrap_or(Screen::Home);
    let out_dir = value_after(&args, "--out").unwrap_or("target/ui-frames");
    let frames = value_after(&args, "--frames")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(1)
        .max(1);
    let backend = value_after(&args, "--backend").unwrap_or("pgm");

    if has_flag(&args, "--serve") {
        return serve(&args, backend, socket);
    }

    if backend == "fbdev" {
        let fb = value_after(&args, "--fb").unwrap_or("/dev/fb0");
        let format = value_after(&args, "--format")
            .and_then(PixelFormat::parse)
            .unwrap_or(PixelFormat::Rgb565);
        let transform = value_after(&args, "--rotate")
            .and_then(Transform::parse)
            .unwrap_or(Transform::None);
        let mut sink = FbdevSink::open_with_transform(fb, format, transform)?;
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

fn serve(args: &[String], backend: &str, socket: &str) -> io::Result<()> {
    let mut sink: Box<dyn FrameSink> = if backend == "fbdev" {
        let fb = value_after(args, "--fb").unwrap_or("/dev/fb0");
        let format = value_after(args, "--format")
            .and_then(PixelFormat::parse)
            .unwrap_or(PixelFormat::Rgb565);
        let transform = value_after(args, "--rotate")
            .and_then(Transform::parse)
            .unwrap_or(Transform::None);
        Box::new(FbdevSink::open_with_transform(fb, format, transform)?)
    } else {
        let out_dir = value_after(args, "--out").unwrap_or("target/ui-frames");
        Box::new(PgmSink::new(out_dir)?)
    };

    let _ = fs::remove_file(socket);
    let listener = UnixListener::bind(socket)?;
    fs::set_permissions(socket, fs::Permissions::from_mode(0o666))?;
    listener.set_nonblocking(true)?;

    let mut state = UiState::new();
    let mut frame = 0;
    loop {
        match listener.accept() {
            Ok((mut stream, _addr)) => {
                let mut buf = String::new();
                stream.read_to_string(&mut buf)?;
                if let Some(command) = Command::parse(&buf) {
                    state.apply(command);
                    stream.write_all(b"OK\n")?;
                } else {
                    stream.write_all(b"ERR unknown command\n")?;
                }
            }
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => {}
            Err(err) => return Err(err),
        }

        let ctx = FrameContext::new(frame, 24).with_selected(state.selected);
        let canvas = render_state(&state, ctx);
        sink.write_frame(state.screen.name(), frame, &canvas)?;
        frame = frame.wrapping_add(1);
        thread::sleep(Duration::from_millis(120));
    }
}

fn send_command(socket: &str, command: &str) -> io::Result<()> {
    let mut stream = UnixStream::connect(socket).map_err(|err| {
        io::Error::new(
            err.kind(),
            format!(
                "could not connect to Rune UI command socket '{socket}': {err}. \
Start the UI with `rune-ui-demo --serve ...` first."
            ),
        )
    })?;
    stream.write_all(command.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Write)?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    print!("{response}");
    Ok(())
}

fn value_after<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|pair| pair.first().map(String::as_str) == Some(key))
        .and_then(|pair| pair.get(1))
        .map(String::as_str)
}

fn has_flag(args: &[String], key: &str) -> bool {
    args.iter().any(|arg| arg == key)
}
