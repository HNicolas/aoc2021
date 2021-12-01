mod day1;

fn main() {
    let timer = std::time::Instant::now();
    day1::run();
    println!("{}us", timer.elapsed().as_micros());
}
