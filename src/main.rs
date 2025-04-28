mod tokenizer;
mod ctl_parser;
mod ast;
mod robdd;


fn main() {
    let input = r#"AG( (p -> (AF(q & (r | T)))) & (EG(F -> (AX(r)))) )"#;
    let tokens = tokenizer::tokenize(input);
    println!("Tokens: {:?}", tokens);

    let mut parser = ctl_parser::CtlParser::new(tokens);
    let ast = parser.parse();
    println!("AST: {:?}", ast);

    let ast = ast.norm_and_opt();
    println!("normalized AST: {:?}", ast);

}