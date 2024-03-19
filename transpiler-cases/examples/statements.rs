use ts_std::*;
#[allow(clippy::all)]
fn main() {
    let a = 1;
    let b = 3;
    if a == b {
        console.log("equal");
    } else {
        console.log("not equal");
    };
    let mut c = 10;
    while c >= 0 {
        console.log(c);
        c -= 1;
    }
}
