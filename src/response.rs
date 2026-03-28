#[derive(Debug)]
pub enum StatusCode{
    Ok,
    NotFound,
    BadRequest,
    InternalServerError,
    Created,
}

impl StatusCode {
    pub fn code(&self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::Created => 201,
            StatusCode::BadRequest => 400,
            StatusCode::NotFound => 404,
            StatusCode::InternalServerError => 500,
        }
    }
    pub fn reason(&self) -> &str {
        match self {
            StatusCode::Ok => "Ok",
            StatusCode::Created => "Created",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse{
    pub status : StatusCode,
    pub headers : Vec<(String, String)>,
    pub body : String,
}

impl HttpResponse{
    pub fn new(status : StatusCode, body : impl Into<String>) -> Self {
        Self {
            status,
            headers : Vec::new(),
            body : body.into(),
        }
    }

    pub fn header(mut self, key : &str, value : &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    pub fn build(&self) -> String {
        let mut result = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status.code(),
            self.status.reason()
        );

        result.push_str(&format!("Content-Length: {}\r\n", self.body.len()));

        for (key, value) in &self.headers {
            result.push_str(&format!("{} : {}\r\n", key, value));
        }

        result.push_str("\r\n");
        result.push_str(&self.body);

        result
    }
}
