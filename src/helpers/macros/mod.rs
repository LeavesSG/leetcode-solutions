// use super::vec2d_pretty_print;

#[allow(unused_macros)]
#[macro_export]
/// 作者：Spore
/// 链接：https://www.zhihu.com/question/465345382/answer/1944023043
/// 来源：知乎
/// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
macro_rules! vecnd {
  ($([$($inner:tt)*]),+ $(,)?) => {
      vec![$(
          vecnd![$($inner)*]
      ),+]
  };
  ($($t:tt)*) => {
      vec![$($t)*]
  };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! prettyprint {
    ($x:expr) => {
        vec2d_pretty_print($x);
    };
}
