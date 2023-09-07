trait B {
    fn foo(mut a: &String) {
        a.push_str("bar"); //~ ERROR cannot borrow `*a` as mutable, as it is behind an `&` reference
    }
}

pub fn foo<'a>(mut a: &'a String) {
    a.push_str("foo"); //~ ERROR cannot borrow `*a` as mutable, as it is behind an `&` reference
}

struct A {}

impl A {
    pub fn foo(mut a: &String) {
        a.push_str("foo"); //~ ERROR cannot borrow `*a` as mutable, as it is behind an `&` reference
    }
}

fn main() {
    foo(&"a".to_string());
    A::foo(&"a".to_string());
}
