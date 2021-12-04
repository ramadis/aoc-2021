mod files;
// mod ex_1 {
//     pub mod ex_1;
// }
// mod ex_2 {
    // pub mod ex_2;
    // pub mod ex_2b;
// }
mod ex_3 {
    pub mod a;
    pub mod b;
}

fn main() {
    // ex_1::ex_1::run_a();
    // ex_1::ex_1::run_b();
    // ex_2::ex_2::run_a();
    // ex_2::ex_2b::run();
    ex_3::a::run();
    // ex_3::b::run();
}
