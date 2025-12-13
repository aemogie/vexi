use std::marker::PhantomData;

pub fn main() {
    macro_rules! succ {
        ($n:ty) => {<$n as Nat>::Next};
    }
    std::hint::black_box(<
        succ!(succ!(succ!(succ!(Zero))))
    >::doo(0, |x| x + 1));
}

trait Nat: Sized {
    type Next: Nat;
    fn doo<T>(i: T, f: fn(T) -> T) -> T;
}

struct Zero;
impl Nat for Zero {
    type Next = Successor<Self>;
    #[inline(always)]
    fn doo<T>(i: T, _: fn(T) -> T) -> T {
        i
    }
}

struct Successor<Predecessor> where Predecessor: Nat {
    __phantom: PhantomData<Predecessor>
}
impl<Predecessor: Nat> Nat for Successor<Predecessor> {
    type Next = Successor<Self>;
    #[inline(always)]
    fn doo<T>(i: T, f: fn(T) -> T) -> T {
        f(Predecessor::doo(i, f))
    }
}
