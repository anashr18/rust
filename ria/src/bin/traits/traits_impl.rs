use std::fmt;
use std::ops::Add;
use std::time::Duration;
#[derive(Clone)] // Derives Clone for MyDuration
struct MyDuration(Duration);
// trait implementation for add function
// trait implementation for display for MyDuration without question maek while printing
impl fmt::Display for MyDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}s {}ns", self.0.as_secs(), self.0.subsec_nanos())
    }
}
// impl add trait for myduration
impl std::ops::Add for MyDuration {
    type Output = MyDuration;
    fn add(self, rhs: Self::Output) -> Self::Output {
        MyDuration(self.0 + rhs.0)
    }
}

// new trait for ref add type
trait AddRef {
    fn add_ref(&self, other: &Self) -> Self;
}
impl AddRef for MyDuration {
    fn add_ref(&self, other: &Self) -> Self {
        MyDuration(self.0 + other.0)
    }
}
fn custom_add<T: Add<Output = T>>(i: T, j: T) -> T {
    // i + j
    // or can call the metho directly
    i.add(j)
}
fn custom_add_ref<T: AddRef>(i: &T, j: &T) -> T {
    i.add_ref(j)
}
fn main() {
    let d1 = MyDuration(Duration::new(20, 12));
    let d2 = MyDuration(Duration::new(20, 12));
    // directly calling the add method of Add trait
    println!("{}", d1.clone().add(d2.clone()));
    // custom add function call
    // this will move the ownership from d1 to self in add function defined in trait of std::Duration
    // which is called by add method of MyDuration
    // println!("{}", add(d1, d2));
    println!("{}", custom_add_ref(&d1, &d2));
    println!("{}", custom_add(d1, d2));
}
