fn main() {
    let mut in_buf = String::with_capacity(64 * 1024); // 64 kib should be enough, right? RIGHT?
    let mut out_buf = String::with_capacity(64 * 1024);
    let stdin = std::io::stdin();
    let config = oscillating_case::Config::default();

    loop {
        // `BufRead::read_line` do warn about potential buffer overflow attack but... ¯\_(ツ)_/¯
        match stdin.read_line(&mut in_buf) {
            Err(e) => eprintln!("{}", e),
            Ok(0) => break,
            Ok(_) => {
                oscillating_case::oscillate_case_config_buffer(&in_buf, config, &mut out_buf);
                print!("{}", out_buf); // i could do a low-level Write::write stuff here but
                                       // that'd be an overkill i guess?
            }
        }
        in_buf.clear();
        out_buf.clear();
    }
}
