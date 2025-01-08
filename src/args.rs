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
    /// The first argument
    pub first_arg: String,
    /// The second argument
    pub secon_arg: String,
}