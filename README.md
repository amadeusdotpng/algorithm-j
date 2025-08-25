# algorithm-j

An implementation of [Algorithm J](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system#Algorithm_J)
described by J. Roger Hindley and Robin Milner in Rust. It closely follows the judgment rules
presented in the algorithm.

The `src/typck.rs` file contains most of the implementation. The `src/typ.rs` contains the type
definitions used. 

**Note**: this project is still not complete! There are still missing quality of life features I
need to implement.

- [ ] better printing for types 
- [ ] add comments explaining the algorithm and implementation choices

---

## Running

Make sure that `cargo` is installed, then run `cargo run`.

---

## Resources Used

This was a project made for learning more about type-checking and type-inference, here are some
useful resources I used that helped me out!

- [jfecher - algorithm-j](https://github.com/jfecher/algorithm-j)
- [Max Bernstein and River Dillion Keefer - Damas-Hindley-Milner inference two ways](https://bernsteinbear.com/blog/type-inference/)
- [Wikipedia - Hindley-Milner type system](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system#Algorithm_J)
