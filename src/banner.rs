use colored::*;

pub fn show(name: &str, github: &str) {
    println!("{}", r#"




         111111111    11      11    111111011
        11            11      11    11     10
        11            11      11    11     10
         11111111     11      11    111110101
                11    11      11    11
                11    11      11    11
        111111111      10010111     11


        11      11    11      11    11      11
        111     11    11      11    111    010
        11 11   11    11      11    11 1111 10
        11  11  11    11      11    11  10  01
        11   11 11    11      11    11      10
        11     111    11      11    11      01
        11      11     11111111     11      10

"#.bright_green());

    println!("{}", format!("        Institut Supérieur du Numérique — by {}", name).bright_white());
    println!("{}", format!("        GitHub : {}", github).bright_blue());
    println!();
}
