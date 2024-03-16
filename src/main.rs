use std::{
    collections::BTreeMap,
    env, f64,
    fs::File,
    io::{BufRead, BufReader},
};

struct Stats {
    min: f64,
    max: f64,
    sum: f64,
    count: f64,
}

impl Stats {
    fn update(&mut self, next_val: f64) {
        self.count += 1.0;
        self.min = self.min.min(next_val);
        self.max = self.max.max(next_val);
        self.sum += next_val;
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            sum: Default::default(),
            count: Default::default(),
        }
    }
}

fn main() {
    let mut args = env::args().into_iter();
    args.next();

    let file_path = match args.next() {
        Some(path) => path,
        None => panic!("filepath not provided"),
    };

    let f = File::open(file_path).unwrap();
    let f = BufReader::new(f);
    let mut data: BTreeMap<String, Stats> = BTreeMap::new();

    for line in f.lines().flatten().take(1_000_000) {
        if let Some((city, temp)) = line.split_once(';') {
            let temp: f64 = temp.parse().unwrap();

            let stat = data.entry(city.to_string()).or_default();
            stat.update(temp);
        } else {
            continue;
        };
    }

    for (city, stats) in data.into_iter() {
        let avg = if stats.count == 0.0 {
            0.0
        } else {
            stats.sum / stats.count
        };

        println!("{city}: {}/{}/{avg}", stats.min, stats.max)
    }
}
