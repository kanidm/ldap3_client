

use std::fmt;
use std::str::FromStr;

#[derive(Debug, StructOpt)]
enum DumpFormat {
    OpenLDAPMemDump,
}

impl FromStr for DumpFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "openldap_mem_dump" => Ok(DumpFormat::OpenLDAPMemDump),
            _ => Err("Unknown DumpFormat. Valid choices are openldap_mem_dump"),
        }
    }
}

impl fmt::Display for DumpFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DumpFormats: openldap_mem_dump_json")
    }
}

#[derive(Debug, StructOpt)]
struct BerDumpOptions {
    /// The format of the dump.
    ///
    /// * openldap_mem_dump
    /// A formatted array of bytes, taken from an openldap memory dump.
    /// This can be extract from gdb by examining `op->o_ber` from an operation.
    /// Since this has been partially pre-processed by openldap, this is not a full
    /// valid message. An example is `[0x00, 0x01, 0x02, ...]`
    ///
    #[structopt(short, long)]
    format: DumpFormat,

    #[structopt(short, long)]
    /// the path to the dump
    path: PathBuf
}

#[derive(Debug, StructOpt)]
enum LdapDebugAction {
    /// Dump ldap messages from BER dumps
    BerDump(BerDumpOptions),
}


#[derive(Debug, StructOpt)]
#[structopt(author, name="ldap")]
struct LdapDebugOpt {
    #[structopt(short, long)]
    /// Display extended infomation during runtime.
    verbose: bool,

    #[structopt(flatten)]
    /// The ldap action to perform
    action: LdapDebugAction
}

