#[derive(Debug)]
pub enum CarajilloError {
    // 3rd-party
    Ureq(ureq::Error),
    QuickxmlDe(quick_xml::de::DeError),


    // std
    IO(std::io::Error),
    Var(std::env::VarError),
}

impl From<std::io::Error> for CarajilloError {
    fn from(err: std::io::Error) -> CarajilloError {
        CarajilloError::IO(err)
    }
}

impl From<std::env::VarError> for CarajilloError {
    fn from(err: std::env::VarError) -> CarajilloError {
        CarajilloError::Var(err)
    }
}

impl From<quick_xml::de::DeError> for CarajilloError {
    fn from(err: quick_xml::de::DeError) -> CarajilloError {
        CarajilloError::QuickxmlDe(err)
    }
}

impl From<ureq::Error> for CarajilloError {
    fn from(err: ureq::Error) -> CarajilloError {
        CarajilloError::Ureq(err)
    }
}
