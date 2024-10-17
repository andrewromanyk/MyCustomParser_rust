//mod lib;
use my_parser_andrewromanyk::*;

pub fn main() {
    assert_eq!(list_parser::list("[1,1,2,3,5,8]"), Ok(vec![1, 1, 2, 3, 5, 8]));
    println!("{:?}", list_parser::list("[1,2,3,5,8]"));
}