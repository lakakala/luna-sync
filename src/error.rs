pub struct Error {
    code: String,
    msg: String,
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
