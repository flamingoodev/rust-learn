/// Rust Quiz#18
// What is the output of this Rust program?

struct S {
    f: fn(),
}

impl S {
    fn f(&self) {
        print!("1");
    }
}

#[test]
fn test_quiz_18() {
    let print2 = || print!("2");
    S { f: print2 }.f();
    // (S { f: print2 }.f)(); output: 2
}
// The program exhibits undefined behavior
// The program does not compile
// Correct! The program is guaranteed to output:  1

//
// A call that looks like .f() always resolves to a method,
// in this case the inherent method S::f.
// If there were no method f in scope, a call like this would fail to compile even if
// a field f exists and contains a function pointer.
//
// To call the function pointer stored in field f,
// we would need to write parentheses around the field access:
//
// fn main() {
//     let print2 = || print!("2");
//     (S { f: print2 }.f)();
// }
