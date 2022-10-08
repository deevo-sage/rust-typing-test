mod test;

fn main() {
    let info = test::start();
    let input = test::take_input(&info);
    let res = test::get_result(input, info);
    println!("{:?}", res)
}
