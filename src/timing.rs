use std::fmt;
use std::fmt::{Debug, Formatter};
use std::time::{Duration, Instant};

pub struct BenchmarkResult {
  times: u32,
  total: Duration,
  min: Duration,
  max: Duration,
  avg: Duration,
  // mean: Duration,
}

impl Debug for BenchmarkResult {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_tuple("")
      .field(&self.min)
      .field(&self.avg)
      .field(&self.max)
      .field(&self.total)
      .field(&self.times)
      .finish()
  }
}

pub fn benchmark<T>(times: u32, callback: impl Fn() -> T) -> (T, BenchmarkResult) {
  let mut timer_list = vec![];
  for _ in 0..times {
    let start = Instant::now();
    callback();
    timer_list.push(start.elapsed().as_nanos())
  }
  let sum = timer_list.iter().sum::<u128>();
  let min = timer_list.iter().min().unwrap();
  let max = timer_list.iter().max().unwrap();
  let avg = sum / times as u128;
  // let mean = (min + max) / 2;

  let result = callback();
  (
    result,
    BenchmarkResult {
      times,
      total: Duration::from_nanos(sum as u64),
      min: Duration::from_nanos(*min as u64),
      max: Duration::from_nanos(*max as u64),
      avg: Duration::from_nanos(avg as u64),
      // mean: Duration::from_nanos(mean as u64),
    },
  )
}
