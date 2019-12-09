use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "is-armstrong")]
struct Opt {
    #[structopt(required = true)]
    num: u32,
}

fn is_armstrong(num: u32) -> bool {
    let length = num.to_string().len() as u32;

    num == num
        .to_string()
        .chars()
        .fold(0, |acc, c| acc + c.to_digit(10).unwrap().pow(length))
}

fn main() {
    let num = Opt::from_args().num;

    if is_armstrong(num) {
        println!("{} is an Armstrong number!", num);
    } else {
        println!("{} is not an Armstrong number...", num);
    }
}
