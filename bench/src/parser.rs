extern crate test;

#[bench]
fn bench_parse_println(b: &mut test::Bencher) {
  b.iter(|| println!("hello world!"));
}
