use std::{env::args, fs::File, io::Read};

fn main() {
    let args: Vec<String> = args().collect();
    assert_eq!(args.len(), 2, "no args");
    let filename = &args[1];

    let mut file = File::open(filename).expect("file not found");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let cs: Vec<String> = buf
        .iter()
        .map(|v| {
            if let Ok(c) = std::str::from_utf8(&[*v]) {
                c.to_string()
            } else {
                format!("<{}>", v)
            }
        })
        .collect();
    println!("{}", cs.join(""));
}
