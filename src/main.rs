mod parse;

mod typ;

mod ctx;
use ctx::TypeContext;

mod typck;

use std::io;
use std::io::Write;

fn main() {
    let mut buf = String::new();
    let stdin = io::stdin();
    loop {
        print!(">> ");
        let _ = io::stdout().flush();
        if let Err(e) = stdin.read_line(&mut buf) {
            eprintln!("{}", e); return;
        }

        let e = match parse::parse(buf.as_str()) {
            Ok(e)  => e,
            Err(e) => { eprintln!("{:?}", e); return; },
        };

        // \f. \x. f (f x) : ('a -> 'a) -> 'a -> 'a
        let mut ctx = TypeContext::new();
        match typck::infer(&mut ctx, &e) {
            Ok(t)  => println!("{} : {}\n", buf.trim(), t),
            Err(e) => eprintln!("{}", e),
        }

        buf.clear();
    }
}
