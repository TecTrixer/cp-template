use lib::*;

fn main() {
    let mut io = Io::new();
    // let t = r!(io, U);
    // for _ in 0..t {

    // }
}

mod lib {
    #![allow(dead_code)]
    pub use std::collections::{HashMap, HashSet};
    use std::{
        fmt::Display,
        io::{
            stdin, stdout, BufRead, BufReader, BufWriter, Cursor, Error, ErrorKind, Read, Stdin,
            Stdout, Write,
        },
        str::{from_utf8_unchecked, FromStr},
    };
    pub type U = usize;
    pub type I = isize;
    pub type F = f64;
    pub type B = u8;
    fn is_skip_char(&b: &u8) -> bool {
        b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' || b == b','
    }
    pub struct Io<R, W>
    where
        R: Read,
        W: Write,
    {
        input: BufReader<R>,
        output: BufWriter<W>,
    }
    impl Io<&[u8], Stdout> {
        #[allow(clippy::should_implement_trait)]
        /// This function creates an io handler from a &str which can be used to make parsing easier.
        pub fn from_str(input: &str) -> Io<&[u8], Stdout> {
            Io {
                input: BufReader::new(input.as_bytes()),
                output: BufWriter::new(stdout()),
            }
        }
        /// This function creates an io handler from a String which can be used to parse lines easier.
        pub fn from_string(input: String) -> Io<Cursor<String>, Stdout> {
            Io {
                input: BufReader::new(Cursor::new(input)),
                output: BufWriter::new(stdout()),
            }
        }
    }
    impl Io<Stdin, Stdout> {
        /// This functions creates the default I/O handler using stdin and stdout as reader and writer.
        pub fn new() -> Io<Stdin, Stdout> {
            Io {
                input: BufReader::new(stdin()),
                output: BufWriter::new(stdout()),
            }
        }
    }
    impl<R: std::io::Read, W: std::io::Write> Io<R, W> {
        pub fn with_reader_and_writer(reader: R, writer: W) -> Io<R, W> {
            Io {
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
        pub fn line_io(&mut self) -> impl std::iter::Iterator<Item = Io<Cursor<String>, Stdout>> {
            let file = self.read_all();
            file.lines()
                .map(move |line| Io::from_string(line.to_string()))
                .collect::<Vec<Io<Cursor<String>, Stdout>>>()
                .into_iter()
        }
        pub fn blocks(&mut self) -> Vec<Io<Cursor<String>, Stdout>> {
            let file = self.read_all();
            file.split("\n\n")
                .map(move |line| Io::from_string(line.to_string()))
                .collect::<Vec<Io<Cursor<String>, Stdout>>>()
        }
        pub fn split(&mut self, pattern: &str) -> Vec<Io<Cursor<String>, Stdout>> {
            let file = self.read_all();
            file.split(pattern)
                .map(move |line| Io::from_string(line.to_string()))
                .collect::<Vec<Io<Cursor<String>, Stdout>>>()
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
        ($io:expr, $($arg:tt)*) => {
            $io.w(format!($($arg)*));
            $io.nl();
        };
    }
    #[macro_export]
    macro_rules! w {
        ($io:expr, $v:expr) => {
            $io.w($v);$io.nl()
        };
        ($io:expr, $($v:expr);*, $l:expr) => {
            $(
                $io.w($v);
                $io.w(' ');
            )*
            $io.w($l);
            $io.nl()
        };
        ($io:expr, $($v:expr),*) => {
            $(
                $io.w($v);
                $io.w(' ');
            )*
            $io.nl()
        }
    }
    #[macro_export]
    macro_rules! r {
        ($io:expr, $T:ty) => {
            $io.r::<$T>()
        };
        ($io:expr, $($T:ty),*) => {
            ($(
                $io.r::<$T>()
            ),*)
        }
    }
}
