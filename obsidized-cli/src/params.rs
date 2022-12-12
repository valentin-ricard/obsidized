use clap::Parser;

#[derive(Parser)] // requires `derive` feature
pub enum Parameters {
    CompileOne {
        /// Input path to the .md file to compile
        path: String,
        /// Output path for the compiled HTML file
        #[arg(short = 'o',
        value_name = "output_file",
        value_hint = clap::ValueHint::FilePath,
        default_value = "output.html")]
        output: String,
        #[arg(short = 'O', default_value="false")]
        overwrite: bool,
    },
}
