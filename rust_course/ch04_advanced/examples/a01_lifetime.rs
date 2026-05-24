use std::f64::consts;

fn main() {
    example1();
    example2();
    unbound_lifetime();
}

// 不太聪明的生命周期检查
fn example1() {
    // #[derive(Debug)]
    // struct Foo;

    // impl Foo {
    //     fn mutate_and_stare<'a>(&'a mut self) -> &'a Self {
    //         &*self
    //     }
    //     fn share(&self) {}
    // }

    // let mut foo = Foo;
    // let loan = foo.mutate_and_stare();
    // foo.share();            // cannot borrow `foo` as immutable because it is also borrowed as mutable
    // println!("{:?}", loan);
}

// 不太聪明的生命周期检查
fn example2() {
    // use std::collections::HashMap;
    // use std::hash::Hash;

    // fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V 
    //     where 
    //         K: Clone + Eq + Hash,
    //         V: Default
    // {
    //     match map.get_mut(&key) {
    //         Some(value) => value,
    //         None => {
    //             map.insert(key.clone(), V::default());   // cannot borrow `*map` as mutable more than once at a time
    //             map.get_mut(&key).unwrap()               // cannot borrow `*map` as mutable more than once at a time
    //         }
    //     }
    // }
}

// 无界生命周期
fn unbound_lifetime() {
    fn f<'a, T>(x: *const T) -> &'a T {
        unsafe {
            &*x
        }
    }
}