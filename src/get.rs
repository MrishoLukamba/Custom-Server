use std::{fmt, fs};
use std::fmt::{Display, Formatter, write};

#[derive(Debug, Clone)]
pub enum getErrors{
    RequestFailed,
    FileNotFound,

}
impl fmt::Display for getErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            getErrors::RequestFailed => write!(f, "Request Failed"),
            getErrors::FileNotFound => write!(f,"File Not Found"),
        }
    }
}

pub fn get_method(params:&str) -> String{
    match params {
        "/" => match fs::read_to_string("./database/Home/home.html"){
            Ok(file) => file,
            _=> getErrors::FileNotFound.to_string()
          },


          _ => {
                let params_extra:Vec<&str> = params.split("/").collect();
                match fs::read_to_string(params_extra[0]){
                    Ok(file) =>  file,
                    _=> getErrors::FileNotFound.to_string()
                }
             }
    }

}