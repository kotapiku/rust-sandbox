// ctrl-dが押されるまで文字列を読み続け，行ごとに反転して返す

use std::io::Read;
use std::str;

fn reserve_each_line(text: &str) -> String {
    text.split_terminator("\n")
        .flat_map(|line| line.chars().rev().chain(Some('\n')))
        .collect()
}

fn main() {
    let mut scan = std::io::stdin();
    let mut buf = Vec::new();
    scan.read_to_end(&mut buf).unwrap();
    let s = str::from_utf8(&buf[..]).unwrap();
    print!("{}", reserve_each_line(s));
}
