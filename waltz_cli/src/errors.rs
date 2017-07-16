error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Cli(::structopt::clap::Error);
        Logging(::log::SetLoggerError);
        Waltz(::waltz::Error);
    }
}
