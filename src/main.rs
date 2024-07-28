use std::{
    fs::File,
    io::{Error, Read},
};

use crusty::{generator::generate, lexer::lex, parser::parse};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 1 {
        eprintln!("Error invalid number of arguments :");
        std::process::exit(1);
    }

    let tokens = match read_file(&args[0]) {
        Ok(s) => lex(&s),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let ast = parse(&tokens);
    let generated = generate(&ast);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    println!("{}", generated);
}

fn read_file(input: &str) -> Result<String, Error> {
    let mut file = File::open(input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
