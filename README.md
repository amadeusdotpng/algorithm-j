# algorithm-j

An implementation of [Algorithm J](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system#Algorithm_J)
described by J. Roger Hindley and Robin Milner in Rust. It closely follows the judgment rules
presented in the algorithm.

The `src/typck.rs` file contains most of the implementation. The `src/typ.rs` contains the type
definitions used. 

**Note**: This project was for learning purposes and is probably really
inefficient (probably used `Rc` way too much and I'm not sure if `RefCell` is any good for
representing graphs).

---

## Running

Make sure that `cargo` is installed, then run `cargo run`.

---

## Resources Used

Here are some useful resources I used that helped me out.

- [jfecher - algorithm-j](https://github.com/jfecher/algorithm-j)
- [Max Bernstein and River Dillion Keefer - Damas-Hindley-Milner inference two ways](https://bernsteinbear.com/blog/type-inference/)
- [Wikipedia - Hindley-Milner type system](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system#Algorithm_J)
