use clap::{crate_version, App, AppSettings, Arg};

pub fn build_app() -> App<'static> {
    App::new("rusty-gql")
        .version(crate_version!())
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand(
            App::new("new")
                .arg(Arg::new("name").required(true).index(1))
                .arg(Arg::new("lib").long("lib").takes_value(true)),
        )
        .subcommand(App::new("generate").alias("g"))
}
