use ldapcli::*;
use structopt::StructOpt;

include!("./cldap_opt.rs");

#[tokio::main(flavor = "current_thread")]
async fn main() {
    ldapcli::start_tracing();
    trace!("cldap command line utility");

    let opt = CldapOpt::from_args();
}
