
#[derive(Debug, StructOpt)]
struct WhoamiOptions {
    // /// The username (distinguished name) to authenticate as
    // bind_dn: String
}

#[derive(Debug, StructOpt)]
enum LdapAction {
    /// Search a directory server
    Search,
    /// Check authentication (bind) to a directory server
    Whoami(WhoamiOptions)
}

#[derive(Debug, StructOpt)]
#[structopt(author, name="ldap")]
struct LdapOpt {
    #[structopt(short, long)]
    /// Display extended infomation during runtime.
    verbose: bool,

    #[structopt(short = "H", long = "url")]
    url: url::Url,

    #[structopt(short = "j", long = "json")]
    json: bool,

    #[structopt(short = "D", long = "dn")]
    bind_dn: Option<String>,

    #[structopt(short = "w", long = "pass")]
    bind_passwd: Option<String>,

    #[structopt(flatten)]
    /// The ldap action to perform
    action: LdapAction
}

