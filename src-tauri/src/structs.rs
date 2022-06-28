use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct Word {
    pub(crate) word:String,
    pub(crate) language:String,
    pub(crate) type_:String,
    pub(crate) group:String,
    pub(crate) size:String,
}
