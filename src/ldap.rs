use ldapcli::*;
use structopt::StructOpt;

include!("./ldap_opt.rs");

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let opt = LdapOpt::from_args();
    ldapcli::start_tracing(opt.verbose);
    info!("ldap command line utility");

    let timeout = Duration::from_secs(1);

    let (bind_dn, bind_passwd) = if let Some(dn) = opt.bind_dn {
        if let Some(pw) = opt.bind_passwd {
            (dn.clone(), pw.clone())
        } else if opt.json {
            let e = LdapError::PasswordNotFound;
            println!(
                "{}",
                serde_json::to_string_pretty(&e).expect("CRITICAL: Serialisation Fault")
            );
            std::process::exit(e as i32);
        } else {
            let pw =
                match rpassword::prompt_password_stderr(&format!("Enter password for {}: ", dn)) {
                    Ok(p) => p,
                    Err(e) => {
                        error!("Failed to get bind password - {}", e);
                        std::process::exit(LdapError::PasswordNotFound as i32);
                    }
                };
            (dn.clone(), pw)
        }
    } else {
        if opt.bind_passwd.is_some() {
            let e = LdapError::AnonymousInvalidState;
            if opt.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&e).expect("CRITICAL: Serialisation Fault")
                );
            } else {
                error!("Anonymous does not take a password - {}", e);
            }
            std::process::exit(e as i32);
        } else {
            ("".to_string(), "".to_string())
        }
    };

    let mut client = match LdapClient::new(&opt.url, timeout).await {
        Ok(c) => c,
        Err(e) => {
            if opt.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&e).expect("CRITICAL: Serialisation Fault")
                )
            } else {
                error!("Failed to create ldap client - {}", e);
            }
            std::process::exit(e as i32);
        }
    };

    // The first message after connect is always a bind.
    if let Err(e) = client.bind(bind_dn, bind_passwd).await {
        if opt.json {
            println!(
                "{}",
                serde_json::to_string_pretty(&e).expect("CRITICAL: Serialisation Fault")
            )
        } else {
            error!("Failed to create ldap client - {}", e);
        }
        std::process::exit(e as i32);
    };

    match opt.action {
        LdapAction::Search => {}
        LdapAction::Whoami(options) => {}
    }
}
