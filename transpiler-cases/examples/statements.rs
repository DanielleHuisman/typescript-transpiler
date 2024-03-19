use ts_std::*;
#[allow(clippy::all)]
fn main() {
    let a = 1;
    let b = 2;
    let c = 3;
    if a == b {
        console.log("equal to b");
    } else if a == c {
        console.log("equal to c");
    } else {
        console.log("not equal to b or c");
    };
    console.log("while");
    let mut d = 10;
    while d >= 0 {
        console.log(d);
        d -= 1;
    }
    console.log("do while");
    d = 10;
    loop {
        console.log(d);
        d -= 1;
        if !(d >= 0) {
            break;
        }
    }
    console.log("for");
    for i in (0..=10).rev().step_by(2) {
        console.log(i);
    }
}
