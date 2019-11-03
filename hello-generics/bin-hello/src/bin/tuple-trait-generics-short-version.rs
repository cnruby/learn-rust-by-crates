// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=f43476780dd8328fe4bea7da31a06be4
#[derive(Default, PartialEq)]
struct ____();

trait __<T> {
    fn __(&self) -> ();
}

impl<T, U> __<T> for U {
    fn __(&self) -> () {
        ()
    }
}

fn __<___, U>(__: ___) -> ()
where
    ___: __<____>,
    ___: std::ops::Deref<Target = U>,
    U: __<____> + ?Sized,
{
    __.__()
}

fn main() {
    let ___: ____ = Default::default();
    assert_eq!((), __(&___));
    let ___: Box<_> = Box::new(___);
    assert_eq!((), __(___));
}
