// Assembly info
pub const VERSION: &str = "0.1.0";
pub const AUTHOR: &str = "Alexankitty";
pub const PROJECT_NAME: &str = "rm-trash";

pub const ERROR_TEMPLATE: &str = "{error}";
pub const REMOVE_ERROR_TEMPLATE: &str = "cannot remove {path}: {error}";

// Question Templates
pub const ACTION_TEMPLATE: &str = "{verb} {kind}: {path}";
pub const ACTION_TEMPLATE_NO_KIND: &str = "{verb}: {path}";

// Kinds
pub const KIND_FILE: &str = "file";
pub const KIND_DIR: &str = "directory";
pub const KIND_EMPTY_FILE: &str = "empty file";
pub const KIND_SYMBOLIC_LINK: &str = "symbolic link";

// Verbs
pub const VERB_REMOVE: &str = "remove";
pub const VERB_REMOVED: &str = "removed";
pub const VERB_TRASH: &str = "trash";
pub const VERB_TRASHED: &str = "trashed";

pub const VERB_DESCEND: &str = "descend into";
pub const VERB_DESCENDING: &str = "descending into";

// Error Reasons
pub const ERROR_REASON_NO_SUCH_FILE: &str = "no such file or directory";
pub const ERROR_REASON_IS_DIR: &str = "is a directory";
pub const ERROR_OPERAND: &str = "missing operand";

// Strings
pub const HELP: &str = "--help' for more information.";
pub const USAGE: &str = "[OPTIONS]... [FILE]...";
pub const HELP_TEXT: &str = "

    Argument compatible implementation of rm, but to your trash directory.
    Behaviors are not identical to rm due to the trash directory implementation.
    Some return codes may be different from rm.

rm arguments:
    -f, --force
           ignore nonexistent files and arguments, never prompt
    -i
           prompt before every removal
    -I
           prompt once before removing more than three files,
           or when removing recursively; less intrusive than -i,
           while still giving protection against most mistakes
        --interactive[=WHEN]
           prompt according to WHEN: never, once (-I), or always (-i);
           without WHEN, prompt always
        --one-file-system
           when removing a hierarchy recursively,
           skip any directory that is on a file system different
           from that of the corresponding command line argument
        --no-preserve-root
           do not treat '/' specially
        --preserve-root[=all]
           do not remove '/' (default);
           with 'all', reject any command line argument
           on a separate device from its parent
    -r, -R, --recursive
           remove directories and their contents recursively
    -d, --dir
           remove empty directories
    -v, --verbose
           explain what is being done
        --help
           display this help and exit
        --version
           output version information and exit
        --no-trash
           do not use the trash directory, remove files directly
        --dry-run
           do not remove files, just print what would be done

trash management arguments:
        --empty
           empty the trash directory
        --restore
           restore files from the trash directory
        --list
           show the contents of the trash directory
        --oprhans
           show files missing a .trashinfo metadata file

By default, rm does not remove directories.  Use the --recursive (-r or -R)
option to remove each listed directory, too, along with all of its contents.

Any attempt to remove a file whose last file name component is '.' or '..'
is rejected with a diagnostic.

To remove a file whose name starts with a '-', for example '-foo',
use one of these commands:
rm -- -foo

rm ./-foo

If you use rm to remove a file, it might be possible to recover
some of its contents, given sufficient expertise and/or time.  For greater
assurance that the contents are unrecoverable, consider using shred(1).

Report bugs at: https://github.com/alexankitty/rm-trash/issues
rm-trash homepage: https://github.com/alexankitty/rm-trash
    ";
