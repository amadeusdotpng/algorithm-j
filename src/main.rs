mod parse;

mod typ;

mod ctx;
use ctx::TypeContext;

mod typck;

fn main() {
    let src = r"\f. \x. f (f x)";
    let e = match parse::parse(src) {
        Ok(e)  => e,
        Err(e) => { eprintln!("{:?}", e); return; },
    };

    // \f. \x. f (f x) : ('a -> 'a) -> 'a -> 'a
    let mut ctx = TypeContext::new();
    match typck::infer(&mut ctx, &e) {
        Ok(t)  => println!("{} : {}", src, t),
        Err(e) => eprintln!("{}", e),
    }
}
