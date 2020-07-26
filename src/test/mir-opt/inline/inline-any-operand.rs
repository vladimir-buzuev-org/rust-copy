// compile-flags: -Z span_free_formats

// Tests that MIR inliner works for any operand

fn main() {
    println!("{}", bar());
}

// EMIT_MIR rustc.bar.Inline.after.mir
fn bar() -> bool {
    let f = foo;
    f(1, -1)
}

#[inline(always)]
fn foo(x: i32, y: i32) -> bool {
    x == y
}
