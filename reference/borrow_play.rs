trait Test {
    fn f(&self) -> u32;
}

trait Holder {
    fn print_one(&self);
}

struct Holder2<T: Test> {
    one: T,
}

impl<T: Test + std::fmt::Debug> Holder for Holder2<T> {
    fn print_one(&self) {
        println!("{:?}", self.one);
    }
}

struct Holder1<'a, T> where T: 'a + Test + std::fmt::Debug {
    one: &'a T,
}

impl<'a, T: 'a + Test + std::fmt::Debug> Holder for Holder1<'a, T> {
    fn print_one(&self) {
        println!("{:?}", self.one)
    }
}

#[derive(Debug)]
struct TestImpl(u32);

impl Test for TestImpl {
    fn f(&self) -> u32 {
        match self {
            &TestImpl(n) => n
        }
    }
}

fn main() {
    let t = TestImpl(10);
    let t2 = TestImpl(20);
    let h1 = Holder1 { one: &t };
    let h2 = Holder2 { one: t2 };

    h1.print_one();
    h2.print_one();
}
