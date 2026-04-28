use std::env;

#[derive(Debug)]
pub struct Options {
    pub recursive: bool,
    pub force: bool,
    pub files: Vec<String>,
    pub when: i32,
    pub one_file_system: bool,
    pub preserve_root: bool,
    pub preserve_all_root: bool,
    pub empty_dir: bool,
    pub verbose: bool,
    pub no_trash: bool,
    pub help: bool,    //trigger help text
    pub version: bool, //trigger version text
    pub dry_run: bool,
}

pub fn parse_args() -> Options {
    let mut args = Options {
        recursive: false,
        force: false,
        files: Vec::new(),
        when: 0,
        one_file_system: false,
        preserve_root: false,
        preserve_all_root: false,
        empty_dir: false,
        verbose: false,
        no_trash: false,
        help: false,
        version: false,
        dry_run: false,
    };
    let mut args_handled = false;
    let sys_arg = env::args().skip(1);
    // todo: handle platforms that don't populate first arg?
    for argument in sys_arg {
        if argument == "--" {
            args_handled = true;
            continue;
        } else if args_handled {
            args.files.push(argument)
        } else if argument.starts_with('-') && !argument.starts_with("--") {
            for char in argument.chars() {
                match char {
                    'r' | 'R' => args.recursive = true,
                    'f' => args.force = true,
                    'i' => args.when = 2,
                    'I' => args.when = 1,
                    'd' => args.empty_dir = true,
                    'v' => args.verbose = true,
                    _ => {}
                }
            }
        } else if argument.starts_with("--") {
            let split_param = argument.split("=").collect::<Vec<&str>>();
            match split_param[0] {
                "--interactive" => match split_param[1] {
                    "always" => args.when = 2,
                    "once" => args.when = 1,
                    "never" => args.when = 0,
                    _ => args.when = 2,
                },
                "--preserve-root" => {
                    args.preserve_root = true;
                    if split_param[1] == "all" {
                        args.preserve_all_root = true;
                    }
                }
                _ => {}
            }

            match split_param[0] {
                "--help" => args.help = true,
                "--version" => args.version = true,
                "--verbose" => args.verbose = true,
                "--recursive" => args.recursive = true,
                "--interactive" => args.when = 2,
                "--force" => args.force = true,
                "--dir" => args.empty_dir = true,
                "--no-preserve-root" => args.preserve_root = false,
                "--one-file-system" => args.one_file_system = true,
                "--no-trash" => args.no_trash = true,
                "--dry-run" => args.dry_run = true,
                _ => {}
            }
        } else {
            args.files.push(argument)
        }
    }
    args
}
