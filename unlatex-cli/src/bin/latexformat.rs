use anyhow::Result;
use gumdrop::Options;
use unlatex::format_with_opts;
use std::{io::{self, Read, Write}, fs};

#[derive(Debug, Options)]
struct UnLaTexOptions {
    #[options(free, help = "input files [default: stdin]")]
    files: Vec<String>,
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "working directory [default: current directory]")]
    workdir: Option<String>,
    #[options(help = "whether to overwrite existing files [default: false]", short = "w")]
    overwrite: bool,
    #[options(help = "output file [default: stdout]")]
    output: Option<String>,
    #[options(help = "maximum line length [default: 120]", default = "120")]
    print_width: i32,
    #[options(help = "whether to use tabs for indentation [default: false]")]
    use_tabs: bool,
    #[options(help = "number of spaces to use for indentation [default: 2]", default = "2")]
    tab_width: i32,
    #[options(help = "whether to only format the document body [default: false]", short = "d")]
    document_only: bool,
}

fn main() -> Result<()> {
    let opts = UnLaTexOptions::parse_args_default_or_exit();
    if opts.help {
        println!("{}", UnLaTexOptions::usage());
    } else {
        if let Some(workdir) = opts.workdir {
            std::env::set_current_dir(workdir)?;
        }
        // get the thing we can read from
        match (opts.files.len(), opts.output) {
            (0, Some(output)) => {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                let formatted = format_with_opts(&buffer, opts.print_width, opts.use_tabs, opts.tab_width, opts.document_only)?;
                let mut f = fs::File::create(output)?;
                f.write_all(formatted.as_bytes())?;
            }
            (0, None) => {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer).unwrap();
                let output = format_with_opts(&buffer, opts.print_width, opts.use_tabs, opts.tab_width, opts.document_only)?;
                io::stdout().write_all(output.as_bytes())?;
            }
            _ => {
                for file in opts.files {
                    let mut buffer = String::new();
                    let mut f = fs::File::open(&file)?;
                    f.read_to_string(&mut buffer)?;
                    let formatted = format_with_opts(&buffer, opts.print_width, opts.use_tabs, opts.tab_width, opts.document_only)?;

                    if opts.overwrite {
                        let mut f = fs::File::create(&file)?;
                        f.write_all(formatted.as_bytes())?;
                    } else {
                        io::stdout().write_all(formatted.as_bytes())?;
                    }
                }
            }
        };
    };

    Ok(())
}