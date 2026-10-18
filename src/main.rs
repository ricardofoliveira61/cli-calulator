use calculator::evaluator::evaluate;
use calculator::lexer::tokenize;
use calculator::parser::Parser;
use std::env;
use std::io::Write;

fn calculate(expression: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let tokens = tokenize(expression)?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let result = evaluate(&ast);

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();

    args.next();

    let mut stdout = std::io::stdout().lock();

    for expression in args {
        let result = calculate(&expression)?;

        writeln!(stdout, "{expression} : {result}")?;
    }

    Ok(())
}
