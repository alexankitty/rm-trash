mod arguments;
mod prettyprint;

use crate::prettyprint::constants::*;
use crate::prettyprint::printer::PrettyPrinter;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let printer = PrettyPrinter::new();
    let options = arguments::parse_args();
    if options.verbose {
        println!("{:?}", options);
    }
    if options.version {
        printer.print_version();
        return Ok(());
    }
    if options.help {
        printer.print_help();
        return Ok(());
    }
    if options.files.is_empty() {
        printer.print_help_short();
        return Err(OPERAND_ERR.into());
    }
    return Ok(());
}
