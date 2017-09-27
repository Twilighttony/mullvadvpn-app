use {Command, Result};
use clap;

use mullvad_types::account::{AccountData, AccountToken};
use rpc;

pub struct Account;

impl Command for Account {
    fn name(&self) -> &'static str {
        "account"
    }

    fn clap_subcommand(&self) -> clap::App<'static, 'static> {
        clap::SubCommand::with_name(self.name())
            .about("Control and display information about your Mullvad account")
            .setting(clap::AppSettings::SubcommandRequired)
            .subcommand(
                clap::SubCommand::with_name("set")
                    .about("Change account")
                    .arg(
                        clap::Arg::with_name("token")
                            .help("The Mullvad account token to configure the client with")
                            .required(true),
                    ),
            )
            .subcommand(
                clap::SubCommand::with_name("get")
                    .about("Display information about the currently configured account"),
            )
            .subcommand(
                clap::SubCommand::with_name("unset")
                    .about("Removes the account number from the settings"),
            )
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<()> {
        if let Some(set_matches) = matches.subcommand_matches("set") {
            let token = value_t_or_exit!(set_matches.value_of("token"), String);
            self.set(Some(&token))
        } else if let Some(_matches) = matches.subcommand_matches("unset") {
            self.set(None)
        } else if let Some(_matches) = matches.subcommand_matches("get") {
            self.get()
        } else {
            unreachable!("No account command given");
        }
    }
}

impl Account {
    fn set(&self, token: Option<&str>) -> Result<()> {
        rpc::call("set_account", &[token]).map(|_: Option<()>| if let Some(token) = token {
            println!("Mullvad account \"{}\" set", token);
        } else {
            println!("Mullvad account removed");
        })
    }

    fn get(&self) -> Result<()> {
        let account_token: Option<AccountToken> = rpc::call("get_account", &[] as &[u8; 0])?;
        if let Some(account_token) = account_token {
            let expiry: AccountData = rpc::call("get_account_data", &[&account_token])?;
            println!("Mullvad account: {}", account_token);
            println!("Expires at     : {}", expiry.expiry);
        } else {
            println!("No account configured");
        }
        Ok(())
    }
}
