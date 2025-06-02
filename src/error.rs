use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Unable to read configuration from {}", message))]
    ScriptExec {
        source: std::io::Error,
        message: String,
    },
    #[snafu(display("parse config failed {message}"))]
    ConfigFileRead {
        source: std::io::Error,
        message: String,
    },
    #[snafu(display("parse config failed {message}"))]
    ConfigParse {
        source: serde_yml::Error,
        message: String,
    },
}

// impl std::error::Error for Error {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         None
//     }

//     fn description(&self) -> &str {
//         "description() is deprecated; use Display"
//     }

//     fn cause(&self) -> Option<&dyn std::error::Error> {
//         self.source()
//     }

//     fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}
// }
