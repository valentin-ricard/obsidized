use crate::params::Parameters;
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use obsidized::ast::Document;
use obsidized::converter::convert;
use obsidized::parser::parse;
use std::fs::{read_to_string, File};
use std::io::BufWriter;
use std::path::Path;

mod params;

fn compile_one<'a>(input: String, output: String, overwrite: bool) -> Result<()> {
    // get the input file
    let input_file =
        read_to_string(input).context("There was an error while opening input file.")?;

    // Fonctionne
    let ast = parse(&*input_file)
        .map_err(|_| anyhow!("An error occurred while parsing the markdown file"))?;

    let document = Document {
        frontmatter: "".to_string(),
        contents: ast
    };

    // If the override flag is not true, check if the file exists
    if Path::new(&output).exists() {
        if !overwrite {
            return Err(anyhow!("The output file already exists! Add the -O flag to overwrite"));
        }
    }
    // Convert the AST to the output file:
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output).context("An error occurred while opening output file!")?;


    convert(&document, &mut output_file)
        .context("An error occurred while converting file to HTML")?;
    Ok(())
}

fn main() -> Result<()> {
    let result = Parameters::parse();
    match result {
        Parameters::CompileOne { path, output, overwrite } => {
            compile_one(path, output, overwrite)?;
        }
    }

    Ok(())
}
