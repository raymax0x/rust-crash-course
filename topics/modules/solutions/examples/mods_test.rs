use modules_practice::my;

fn main() {
    my::print();
    my::a::print();
    let s = my::a::build(9);
    let f = my::a::print_foo();
}
