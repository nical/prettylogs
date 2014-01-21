use std::io::buffered::{BufferedReader};
use std::io::{stdin};

static INC_INDENT : &'static str = &"[[+]]";
static DEC_INDENT : &'static str = &"[[-]]";
static INDENT_PATTERN : &'static str = &"    ";

fn main() {
    indent();
}

fn indent() {
    let colors = [
        "\x1b[0;31m",
        "\x1b[0;32m",
        "\x1b[0;33m",
        "\x1b[0;34m",
        "\x1b[0;35m",
        "\x1b[0;36m",
        "\x1b[1;31m",
        "\x1b[1;32m",
        "\x1b[1;33m",
        "\x1b[1;34m",
        "\x1b[1;35m",
        "\x1b[1;36m",
        "\x1b[2;34m",
        "\x1b[2;35m",
    ];

    let reset = "\x1b[0m";

    let mut stdin = BufferedReader::new(stdin());
    let mut indent: i32 = 0;
    for line in stdin.lines() {
        let mut out_line = line.slice(0, line.len());

        if line.starts_with(DEC_INDENT) {
            indent -= 1;
            out_line = out_line.slice(INC_INDENT.len(), out_line.len());
        }

        for _ in range(0, indent) {
            print(INDENT_PATTERN);
        }

        if line.starts_with(INC_INDENT) {
            indent += 1;
            out_line = out_line.slice(DEC_INDENT.len(), out_line.len());
        }

        let mut i = 0;
        let mut buf: ~str = ~"";
        loop {
            if i >= out_line.len() {
                print(buf);
                break;
            }
            if out_line[i] == '[' as u8 && out_line[i+1] == '[' as u8 {
                i += 1;
                print(buf);
                buf = ~"";
            } else if out_line[i] == ']' as u8 && out_line[i+1] == ']' as u8 {
                i += 1;
                if buf.starts_with(&"0x") {
                    // Color hex patterns to make easier to match
                    let c = str_hash(buf) % (colors.len() as u32);
                    print(colors[c]);
                    print(buf);
                    print(reset);
                    buf = ~"";
                } else {
                    print("[[");
                    print(buf);
                    print("]]");
                    buf = ~"";
                }
            } else {
                buf = buf + (out_line[i] as char).to_str();
            }
            i += 1;
        }
    }
}

fn str_hash(src: &str) -> u32 {
    let m = 0x5bd1e995 as u32;
    let r = 24 as i32;
    let mut h = 0 as u32;
    let mut len = src.len();
    let mut i = 0;
    while len >= 4 {
        let mut k: u32 = src[0+i] as u32 + (src[1+i] << 8) as u32 + (src[2+i] << 16) as u32 + (src[3+i] << 24) as u32;
        k *= m;
        k ^= k >> r;
        k *=m;
        h *= m;
        h ^= k;
        i += 4;
        len -= 4;
    }

    if len == 3 { h ^= (src[i+2] << 16) as u32; len -= 1; }
    if len == 2 { h ^= (src[i+1] << 8) as u32; len -= 1; }
    if len == 1 { h ^= src[i] as u32; h *= m; }

    h ^= h >> 13;
    h ^= m;
    h ^= h >> 15;

    return h;
}
