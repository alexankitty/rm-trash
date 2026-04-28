mod arguments;
mod filehandler;
mod prettyprint;

extern crate exitcode;

use crate::prettyprint::{constants::*, printer::PrettyPrinter};

fn main() {
    let printer = PrettyPrinter::new();
    let options = arguments::parse_args();
    let file_handler = filehandler::FileHandler::new();
    if options.verbose {
        println!("{:?}\n", options);
    }
    if options.version {
        printer.print_version();
        return;
    }
    if options.help {
        printer.print_help();
        return;
    }
    if options.files.is_empty() {
        printer.print_error(ERROR_OPERAND);
        printer.print_help_short();
        std::process::exit(exitcode::NOINPUT)
    }
    return;
}
