const SHOULD_PANIC: bool = true;

fn main() {
    if SHOULD_PANIC {
        crash_and_burn();
    } else {
        out_of_bounds();
    }
}

fn crash_and_burn() {
    panic!("crash and burn");
}

fn out_of_bounds() {
    let v = vec![1, 2, 3];
    v[99];
}
