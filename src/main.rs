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

fn test_mut_ref() {
    let mut vec = Vec::new();
    vec.push(1);
    let mut vec1: Vec<i32> = Vec::new();
}

fn copy_vec(from: &Vec<i32>, to: &mut Vec<i32>) {
    for elem in from {
        to.push(*elem);
    }
}