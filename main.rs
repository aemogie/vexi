use std::marker::PhantomData;

fn main() { test() }

#[no_mangle]
#[inline(never)]
pub fn side_effect(i: u32) {
    println!("{i}");
}

#[no_mangle]
#[inline(never)]
pub fn test() {
    macro_rules! succ {
        ($n:ty) => {<$n as Nat>::Next};
    }
    let (_, _result) = std::hint::black_box(<
        succ!(succ!(succ!(succ!(Zero))))
    >::repeat(|x| {
        side_effect(x);
        x * x
    }, 2 * 2));
}

trait Nat: Sized {
    type Next: Nat;
    fn repeat<T, F: Fn(T) -> T>(f: F, t: T) -> (F, T);
}

struct Zero;
impl Nat for Zero {
    type Next = Successor<Self>;
    #[inline(always)]
    fn repeat<T, F: Fn(T) -> T>(f: F, t: T) -> (F, T) { (f, t) }
}

struct Successor<Predecessor> where Predecessor: Nat {
    __phantom: PhantomData<Predecessor>
}
impl<Predecessor: Nat> Nat for Successor<Predecessor> {
    type Next = Successor<Self>;
    #[inline(always)]
    fn repeat<T, F: Fn(T) -> T>(f: F, t: T) -> (F, T) {
        let (f, t) = Predecessor::repeat(f, t);
        let t = f(t);
        (f, t)
    }
}
