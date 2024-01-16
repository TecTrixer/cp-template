use lib::*;

fn main() {
    #[allow(unused_variables, unused_mut)]
    let mut io = Io::new();
    // let t = r!(U);
    // for _ in 0..t {

    // }
}

#[rustfmt::skip]
mod lib {
#![allow(dead_code)]
pub use std::collections::{HashMap, HashSet};
use std::{
fmt::Display,
io::{
    stdin, stdout, BufRead, BufReader, BufWriter, Cursor, Error, ErrorKind, Read, Stdin,
    Stdout, Write,
},
str::{from_utf8_unchecked, FromStr}, ops::Deref,
};
pub static mut IO: Option<IoInner<Stdin, Stdout>> = None;
pub type U = usize;
pub type I = isize;
pub type F = f64;
pub type B = u8;
fn is_skip_char(&b: &u8) -> bool {
    b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' || b == b','
}
pub struct IoInner<R, W>
where
    R: Read,
    W: Write,
{
    input: BufReader<R>,
    output: BufWriter<W>,
}
pub struct Io;
impl std::ops::DerefMut for Io {
    fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { IO.as_mut().unwrap_unchecked() }
    }
}
impl Deref for Io {
    type Target = IoInner<Stdin, Stdout>;
    fn deref(&self) -> &Self::Target {
            unsafe { IO.as_mut().unwrap_unchecked() }
    }
}
impl Io {
pub fn new() -> Self {
    unsafe {
        IO = Some(IoInner::new());
    }
    Self
}
}
impl IoInner<&[u8], Stdout> {
#[allow(clippy::should_implement_trait)]
pub fn from_str(input: &str) -> IoInner<&[u8], Stdout> {
    IoInner {
        input: BufReader::new(input.as_bytes()),
        output: BufWriter::new(stdout()),
    }
}
pub fn from_string(input: String) -> IoInner<Cursor<String>, Stdout> {
    IoInner {
        input: BufReader::new(Cursor::new(input)),
        output: BufWriter::new(stdout()),
    }
}
}
impl IoInner<Stdin, Stdout> {
pub fn new() -> IoInner<Stdin, Stdout> {
    IoInner {
        input: BufReader::new(stdin()),
        output: BufWriter::new(stdout()),
    }
}
}
impl Drop for Io {
fn drop(&mut self) {
    self.flush();
}
}
impl<R: Read, W: Write> IoInner<R, W> {
pub fn with_reader_and_writer(reader: R, writer: W) -> IoInner<R, W> {
    IoInner {
        input: BufReader::new(reader),
        output: BufWriter::new(writer),
    }
}
pub fn r<T: FromStr>(&mut self) -> T {
    let buf = self
        .input
        .by_ref()
        .bytes()
        .map(|x| unsafe { x.unwrap_unchecked() })
        .skip_while(is_skip_char)
        .take_while(|c| !is_skip_char(c))
        .collect::<Vec<_>>();
    unsafe { from_utf8_unchecked(&buf) }
        .parse()
        .map_err(|_| Error::new(ErrorKind::Other, "could not parse value"))
        .unwrap()
}
pub fn read_line(&mut self) -> String {
    let mut res = String::new();
    unsafe {
        self.input.read_line(&mut res).unwrap_unchecked();
    }
    res.trim_end().to_string()
}
pub fn read_all(&mut self) -> String {
    let mut res = String::new();
    unsafe { self.input.read_to_string(&mut res).unwrap_unchecked() };
    res
}
pub fn read_char(&mut self) -> char {
    self.input
        .by_ref()
        .bytes()
        .map(|b| b.expect("could not read bytes in io read operation"))
        .find(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t' && b != b',')
        .unwrap() as char
}
pub fn chars(&mut self) -> Vec<char> {
    self.r::<String>().chars().collect()
}
pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
    (0..n).map(|_| self.r::<T>()).collect()
}
pub fn line_io(
    &mut self,
) -> impl std::iter::Iterator<Item = IoInner<Cursor<String>, Stdout>> {
    let file = self.read_all();
    file.lines()
        .map(move |line| IoInner::from_string(line.to_string()))
        .collect::<Vec<IoInner<Cursor<String>, Stdout>>>()
        .into_iter()
}
pub fn blocks(&mut self) -> Vec<IoInner<Cursor<String>, Stdout>> {
    self.split("\n\n")
}
pub fn split(&mut self, pattern: &str) -> Vec<IoInner<Cursor<String>, Stdout>> {
    let file = self.read_all();
    file.split(pattern)
        .map(move |line| IoInner::from_string(line.to_string()))
        .collect::<Vec<IoInner<Cursor<String>, Stdout>>>()
}
pub fn w<T: Display>(&mut self, t: T) {
    unsafe { write!(&mut self.output, "{t}").unwrap_unchecked() };
}
pub fn wl<T: Display>(&mut self, t: T) {
    self.w(t);
    self.nl();
    self.flush();
}
pub fn nl(&mut self) {
    self.w('\n');
}
pub fn flush(&mut self) {
    unsafe { self.output.flush().unwrap_unchecked() }
}
}
#[macro_export]
macro_rules! wf {
($($arg:tt)*) => {
    {
        let io = unsafe {
            IO.as_mut().unwrap_unchecked()
        };
        io.w(format!($($arg)*));
        io.nl();
    }
};
}
#[macro_export]
macro_rules! w {
($v:expr) => {
    {
        let io = unsafe {
            IO.as_mut().unwrap_unchecked()
        };
        io.w($v);io.nl();
    }
};
($($v:expr);*, $l:expr) => {
    {
        let io = unsafe {
            IO.as_mut().unwrap_unchecked()
        };
        $(
            io.w($v);
            io.w(' ');
        )*
        io.w($l);
        io.nl();
    }
};
($($v:expr),*) => {
    {
        let io = unsafe {
            IO.as_mut().unwrap_unchecked()
        };
        $(
            io.w($v);
            io.w(' ');
        )*
        io.nl();
    }
}
}
#[macro_export]
macro_rules! r {
($T:ty) => {
    {
        let io = unsafe {
            IO.as_mut().unwrap_unchecked()
        };
        io.r::<$T>()
    }
};
($($T:ty),*) => {
    {
        let io = unsafe {
        IO.as_mut().unwrap_unchecked()
        };
        ($(
        io.r::<$T>()
        ),*)
    }
}
}
}
