#![feature(const_mut_refs)]
#![feature(const_fn)]
#![feature(raw_ref_op)]

struct Foo {
    x: usize
}

const fn foo() -> Foo {
    Foo { x: 0 }
}

impl Foo {
    const fn bar(&mut self) -> *mut usize {
        &raw mut self.x
    }
}

const fn baz(foo: &mut Foo)-> *mut usize {
    &raw mut foo.x
}

const _: () = {
    foo().bar();
    //~^ ERROR mutable references are not allowed in constants
    baz(&mut foo());
    //~^ ERROR mutable references are not allowed in constants
};

fn main() {}
