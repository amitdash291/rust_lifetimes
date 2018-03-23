fn main() {
    let a = 20;
    let b = 10;
    let result_ref;

    result_ref = which_is_greater(&a, &b)
}

//fn lifetime_violation1() {
//    let a; // 'a lifetime
//
//    {
//        let b = 10; // 'b lifetime
//        a = &b;
//    }
//}
//
//fn lifetime_violation2() {
//    let a; // 'a lifetime
//    let b = 10; // 'b lifetime
//
//    a = &b;
//}

fn which_is_greater<'a, 'b>(a: &'a i32, b: &'b i32) -> &'b i32 {
//    if a > b {
//        a
//    } else {
//        b
//    }
    &(10)
}