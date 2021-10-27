use criterion::{criterion_group, criterion_main, Criterion};
use postcss::{
  input::Input,
  tokenizer::{Token, Tokenizer},
};

const SMALL_CSS_FILE: &str = include_str!("../assets/bootstrap-reboot.css");
const LARGE_CSS_FILE: &str = include_str!("../assets/bootstrap.css");

fn tokenize(css: &str, ignore_errors: bool) -> Vec<Token> {
  let input = Input::new(css.to_string(), None);
  let mut processor = Tokenizer::new(&input, ignore_errors);
  let mut tokens = vec![];
  while !processor.end_of_file() {
    tokens.push(processor.next_token(false))
  }
  return tokens;
}

fn tokenize_bench(c: &mut Criterion) {
  c.bench_function("small css file 7K", |b| {
    b.iter_with_large_drop(|| tokenize(SMALL_CSS_FILE, false));
  });
  c.bench_function("large css file 201K", |b| {
    b.iter_with_large_drop(|| tokenize(LARGE_CSS_FILE, false));
  });
}

criterion_group!(benches, tokenize_bench);
criterion_main!(benches);
