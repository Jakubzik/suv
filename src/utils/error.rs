/// Inspiration from: <https://vincents.dev/blog/rust-errors-without-dependencies/>
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct SuvError {
    pub(crate) kind: Kind,
    // location: &'static Location<'static>,
    pub(crate) description: String,
}

#[derive(Debug)]
pub enum Kind {
    ParseErr(std::num::ParseIntError),
    UserInputErr(std::io::Error),
}

// impl From<std::num::ParseIntError> for SuvError {
//     #[track_caller]
//     fn from(error: std::num::ParseIntError) -> Self {
//         SuvError {
//             kind: SuvErrorKind::ParseErr(error),
//             location: Location::caller(),
//             description: todo!(),
//         }
//     }
// }

impl Error for SuvError {}
impl Display for SuvError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match &self.kind {
            // crate::utils::error::kind::ParseErr(error) => write!(
            //     f,
            //     "Suv function had a parse error {} at location {}",
            //     error.to_string(),
            //     self.location.to_string()
            // ),
            Kind::ParseErr(parse_int_error) => todo!(),
            Kind::UserInputErr(error) => {
                write!(
                    f,
                    "Nutzereingabe nicht verstanden ({}), sorry.",
                    error.to_string(),
                    // self.location.to_string()
                )
            }
        }
    }
}
