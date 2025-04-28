mod tokenizer;
mod ctl_parser;
mod ast;
mod robdd;


fn main() {
    let input = r#"s->AF(ktn)"#;
    let tokens = tokenizer::tokenize(input);
    println!("Tokens: {:?}", tokens);

    let mut parser = ctl_parser::CtlParser::new(tokens);
    let ast = parser.parse();
    println!("AST: {:?}", ast);

    let ast = ast.normalize();
    println!("normalized AST: {:?}", ast);
}