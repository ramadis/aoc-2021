mod files;

fn main() {
    let vec = files::get_lines(String::from("/Users/rama/Documents/adventofcode/2021/rust/src/test.txt"));
    for x in &vec {
        println!("{}", x);
    }
}
