use std::fs::{File, read_to_string};
use std::io::BufWriter;
use clap::Parser;
use crate::params::Parameters;
use anyhow::{Context, Result};
use obsidized::ast::Document;
use obsidized::converter::convert;
use obsidized::parser::parse;

mod params;

fn compile_one<'a>(input: String, output: String) -> Result<()> {
    // get the input file
    let input_file = read_to_string(input)
        .context("There was an error while opening input file.")?;

    let ast = parse(&*input_file)
        .context("An error occurred while parsing the markdown file.")?;

    /*let ast: Document<'a> = Document {
        frontmatter: "".to_string(),
        contents: ast
    };

    // Convert the AST to the output file:
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .truncate(false)
        .open(output).context("An error occurred while opening output file!")?;


    convert(ast, &mut output_file)
        .context("An error occurred while converting file to HTML")?;
    */
    Ok(())
}

fn main() -> Result<()> {
    let result = Parameters::parse();
    match result {
        Parameters::CompileOne { path, output} => {
            compile_one(path, output)?;
        }
    }

    Ok(())
}
