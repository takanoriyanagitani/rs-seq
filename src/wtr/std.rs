use std::io;

use std::io::BufWriter;
use std::io::Write;

pub fn write_ints<I, W>(ints: I, mut wtr: W) -> Result<(), io::Error>
where
    I: Iterator<Item = i64>,
    W: Write,
{
    for i in ints {
        writeln!(&mut wtr, "{i}")?;
    }
    wtr.flush()
}

pub fn ints2stdout<I>(ints: I) -> Result<(), io::Error>
where
    I: Iterator<Item = i64>,
{
    let o = io::stdout();
    let mut ol = o.lock();

    let bw = BufWriter::new(&mut ol);
    write_ints(ints, bw)?;

    ol.flush()
}
