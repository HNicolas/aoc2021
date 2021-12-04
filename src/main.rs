mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let timer = std::time::Instant::now();
    day1::run();
    day2::run();
    day3::run();
    day4::run();
    println!("{}us", timer.elapsed().as_micros());
}
