use solang_parser;

fn main() {
    let file_contents = r#"
    
        pragma solidity ^0.8.16;

        contract SimpleStorage {
            uint x;

            function set(uint newValue) {
                x = newValue;
            }
            
            function get() returns (uint) {
                return x;
            }
        }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    println!("{:?}", source_unit);
}
