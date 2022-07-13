use crate::config::SearchMethod as Method;
use crate::reporter::event::Message;
use crate::Event;

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct FindMsrv {
    search_method: Method,
}

impl FindMsrv {
    pub(crate) fn new(method: Method) -> Self {
        Self {
            search_method: method,
        }
    }
}

impl From<FindMsrv> for Event {
    fn from(it: FindMsrv) -> Self {
        Message::FindMsrv(it).into()
    }
}
