use std::error::Error;
use std::io::Write;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

/// Print a whole chain of errors to stderr.
pub fn print_error(e: &dyn Error) {
    eprintln!();

    let bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    let mut color = ColorSpec::new();
    color.set_fg(Some(Color::Red)).set_bold(true);

    buffer
        .set_color(&color)
        .expect("Error while printing error");
    write!(&mut buffer, "Error").expect("Error while printing error");

    buffer.reset().expect("Error while printing error");
    buffer
        .set_color(ColorSpec::new().set_bold(true))
        .expect("Error while printing error");
    writeln!(&mut buffer, ": {}", e).expect("Error while printing error");

    for source in super::sources(e) {
        buffer
            .set_color(&color)
            .expect("Error while printing error");
        write!(&mut buffer, "   Caused by").expect("Error while printing error");

        buffer.reset().expect("Error while printing error");
        writeln!(&mut buffer, " {}", source).expect("Error while printing error");
    }
    bufwtr.print(&buffer).expect("Error while printing error");
}
