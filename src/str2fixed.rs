use std::io;

use std::io::BufWriter;
use std::io::Write;

use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

pub fn bytes2fixed_us4(mut original: Vec<u8>) -> Vec<u8> {
    original.truncate(4);
    let pad: &[u8] = match original.len() {
        0 => b"________",
        1 => b"_______",
        2 => b"______",
        3 => b"_____",
        _ => b"____",
    };
    original.extend(pad);
    original
}

pub fn bytes2fixed_us2(mut original: Vec<u8>) -> Vec<u8> {
    original.truncate(2);
    let pad: &[u8] = match original.len() {
        0 => b"____",
        1 => b"___",
        _ => b"__",
    };
    original.extend(pad);
    original
}

pub fn bytes2fixed<I, F, W>(source: I, conv: F, mut writer: W) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Vec<u8>, io::Error>>,
    F: Fn(Vec<u8>) -> Vec<u8>,
    W: FnMut(Vec<u8>) -> Result<(), io::Error>,
{
    for i in source {
        let item: Vec<u8> = i?;
        let mapd: Vec<u8> = conv(item);
        writer(mapd)?;
    }
    Ok(())
}

pub fn reader2fixed2writer<R, F, W>(r: R, conv: F, mut w: W, split: u8) -> Result<(), io::Error>
where
    F: Fn(Vec<u8>) -> Vec<u8>,
    R: Read,
    W: Write,
{
    let br = BufReader::new(r);
    let lines = br.split(split);
    let writer = |v: Vec<u8>| {
        w.write_all(&v)?;
        writeln!(&mut w)?;
        Ok(())
    };
    bytes2fixed(lines, conv, writer)
}

pub fn stdin2fixed2stdout<F>(conv: F, split: u8) -> Result<(), io::Error>
where
    F: Fn(Vec<u8>) -> Vec<u8>,
{
    let i = io::stdin();
    let il = i.lock();

    let o = io::stdout();
    let mut ol = o.lock();

    {
        let mut bw = BufWriter::new(&mut ol);
        reader2fixed2writer(il, conv, &mut bw, split)?;
        bw.flush()?;
    }

    ol.flush()
}

pub fn stdin2fixed2stdout_default<F>(conv: F) -> Result<(), io::Error>
where
    F: Fn(Vec<u8>) -> Vec<u8>,
{
    stdin2fixed2stdout(conv, b'\n')
}

pub fn stdin2fixed2stdout_default4() -> Result<(), io::Error> {
    stdin2fixed2stdout_default(bytes2fixed_us4)
}

pub fn stdin2fixed2stdout_default2() -> Result<(), io::Error> {
    stdin2fixed2stdout_default(bytes2fixed_us2)
}

#[non_exhaustive]
pub enum ConversionType {
    /// 4 bytes = 2 bytes + 2 pad bytes
    FixedDword,

    /// 8 bytes = 4 bytes + 4 pad bytes
    FixedQword,
}

pub const CONVERSION_TYPE_DEFAULT: ConversionType = ConversionType::FixedQword;

impl Default for ConversionType {
    fn default() -> Self {
        CONVERSION_TYPE_DEFAULT
    }
}

impl From<&str> for ConversionType {
    fn from(s: &str) -> Self {
        match s {
            "FIXED_DWORD" => Self::FixedDword,
            "FIXED_QWORD" => Self::FixedQword,
            _ => Self::default(),
        }
    }
}

pub fn stdin2fixed2stdout_default_type(t: ConversionType) -> Result<(), io::Error> {
    match t {
        ConversionType::FixedDword => stdin2fixed2stdout_default2(),
        ConversionType::FixedQword => stdin2fixed2stdout_default4(),
    }
}

pub fn stdin2fixed2stdout_default_type_str(conv_typ: &str) -> Result<(), io::Error> {
    stdin2fixed2stdout_default_type(conv_typ.into())
}
