use criterion::{criterion_group, criterion_main, Criterion};

fn fibonacci() {
    let source = "+++++++++++
    >+>>>>++++++++++++++++++++++++++++++++++++++++++++
    >++++++++++++++++++++++++++++++++<<<<<<[>[>>>>>>+>
    +<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<[>++++++++++[-
    <-[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]]>[<<[>>>+<<<
    -]>>[-]]<<]>>>[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]]
    >[<<+>>[-]]<<<<<<<]>>>>>[+++++++++++++++++++++++++
    +++++++++++++++++++++++.[-]]++++++++++<[->-<]>++++
    ++++++++++++++++++++++++++++++++++++++++++++.[-]<<
    <<<<<<<<<<[>>>+>+<<<<-]>>>>[<<<<+>>>>-]<-[>>.>.<<<
    [-]]<<[>>+>+<<<-]>>>[<<<+>>>-]<<[<+>-]>[<+>-]<<<-]";

    let input = "".as_bytes();
    let output = vec![];

    let actual = brainheck::Program::interpret(source, Box::new(input), Box::new(output));
    actual.expect("Program should work");
}

fn bench_fibonacci_interpreter(c: &mut Criterion) {
    c.bench_function("fibonacci", |b| b.iter(|| fibonacci()));
}

criterion_group!(benches, bench_fibonacci_interpreter);
criterion_main!(benches);