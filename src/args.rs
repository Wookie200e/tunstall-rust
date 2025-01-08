use clap::
{
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct TunstallArgs
{
    /// The size of tunstall symbols in bits
    pub tunstall_width: u32,
    /// Name of the specified file 
    pub file_name: String,
}
