/* INPUT
Time:        46     68     98     66
Distance:   358   1054   1807   1080
*/
fn eval(time: f64, distance: f64) -> i64 {
    // -x^2 + time*x - distance > 0
    // (time +- sqrt(time^2 - 4*distance)) / 2
    let low = (time - (time.powi(2) - 4.0 * distance).sqrt()) / 2.0;
    let high = (time + (time.powi(2) - 4.0 * distance).sqrt()) / 2.0;
    let lb = if low.fract() == 0.0 {
        low as i64 + 1
    } else {
        low.ceil() as i64
    };
    let ub = if high.fract() == 0.0 {
        high as i64 - 1
    } else {
        high.floor() as i64
    };
    ub - lb + 1
}
pub fn _day6() {
    let input = vec![(46, 358), (68, 1054), (98, 1807), (66, 1080)];
    let answer: i64 = input
        .into_iter()
        .map(|x| eval(x.0 as f64, x.1 as f64))
        .product();
    println!("day 6 part 1: {}", answer);
    println!("day 6 part 2: {}", eval(46689866f64, 358105418071080f64));
}
