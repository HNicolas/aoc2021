mod day1;
mod day2;
mod day3;

fn main() {
    let timer = std::time::Instant::now();
    day1::run();
    day2::run();
    day3::run();
    println!("{}us", timer.elapsed().as_micros());
}
