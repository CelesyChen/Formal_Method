mod tokenizer4ctl;
mod ctl_parser;
mod ast;
mod robdd;


fn main() {
    let input = r#"AG( (p -> (AF(q & (r | T)))) & (EG(F -> (AX(r)))) )"#;

    let mut parser = ctl_parser::CtlParser::new(tokenizer4ctl::tokenize(input));
    let ast = parser.parse();
    println!("AST: {:?}", ast);

    let ast = ast.norm_and_opt();
    println!("normalized AST: {:?}", ast);

}