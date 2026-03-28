use std::collections::HashMap;

#[derive(Debug)]
pub enum ParseError{
    EmptyRequest,
    InvalidRequestLine,
    InvalidHeader,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyRequest => write!(f, "Empty request"),
            ParseError::InvalidRequestLine => write!(f, "Invalid request line"),
            ParseError::InvalidHeader => write!(f, "Invalid header format"),
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method : String,
    pub path : String,
    pub version : String,
    pub headers : HashMap<String, String>,
    pub body : Option<String>,
}

impl HttpRequest {
    pub fn parse(raw :&str) -> Result<Self, ParseError>{
        let (header_section, body) = match raw.split_once("\r\n\r\n"){
            Some((headers, body)) => (headers, Some(body.to_string())),
            None => (raw, None),
        };

        let mut lines = header_section.lines();

        let req_lines = lines.next().ok_or(ParseError::EmptyRequest)?;
        let mut parts = req_lines.split_whitespace();

        let method = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();
        let path = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();
        let version = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();

        let mut headers = HashMap::new();
        for line in lines{
            let (key, value) = line.split_once(":").ok_or(ParseError::InvalidHeader)?;
            let key = key.trim().to_lowercase();
            let value = value.trim().to_string();
            headers.insert(key, value);
        }

        Ok(Self {method, path, version, headers, body})
    }

    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(&key.to_lowercase())
    }
}
