use core::fmt::Debug;

#[allow(dead_code)]
pub fn vec2d_pretty_print<T: Debug>(mat: &[T]) {
    mat.iter().for_each(|x| println!("{:?}", x));
    println!();
}
