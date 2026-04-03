use crate::ast::Json;
use std::collections::HashMap;

struct Parser<'a> {
    s: &'a str,
    pos: usize,
}

pub fn parse(s: &str) -> Result<Json, String> {
    let mut p = Parser { s, pos: 0 };
    p.skip_ws();
    let v = p.parse_value()?;
    p.skip_ws();
    if p.pos != p.s.len() { Err("trailing characters".into()) } else { Ok(v) }
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<char> { self.s[self.pos..].chars().next() }
    fn next_char(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }
    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() { if c.is_whitespace() { self.next_char(); } else { break } }
    }

    fn parse_value(&mut self) -> Result<Json, String> {
        self.skip_ws();
        match self.peek() {
            Some('n') => self.parse_null(),
            Some('t') | Some('f') => self.parse_bool(),
            Some('"') => self.parse_string().map(Json::String),
            Some('[') => self.parse_array(),
            Some('{') => self.parse_object(),
            Some(c) if c == '-' || c.is_digit(10) => self.parse_number(),
            Some(c) => Err(format!("unexpected char: {}", c)),
            None => Err("unexpected end".into()),
        }
    }

    fn parse_null(&mut self) -> Result<Json, String> {
        if self.s[self.pos..].starts_with("null") { self.pos += 4; Ok(Json::Null) } else { Err("invalid token".into()) }
    }

    fn parse_bool(&mut self) -> Result<Json, String> {
        if self.s[self.pos..].starts_with("true") { self.pos += 4; Ok(Json::Bool(true)) }
        else if self.s[self.pos..].starts_with("false") { self.pos += 5; Ok(Json::Bool(false)) }
        else { Err("invalid boolean".into()) }
    }

    fn parse_string(&mut self) -> Result<String, String> {
        // assume current char is '"'
        self.next_char();
        let mut out = String::new();
        loop {
            match self.next_char() {
                Some('"') => break,
                Some('\\') => {
                    match self.next_char() {
                        Some('n') => out.push('\n'),
                        Some('t') => out.push('\t'),
                        Some('r') => out.push('\r'),
                        Some('\\') => out.push('\\'),
                        Some('"') => out.push('"'),
                        Some(c) => out.push(c),
                        None => return Err("unterminated escape".into()),
                    }
                }
                Some(c) => out.push(c),
                None => return Err("unterminated string".into()),
            }
        }
        Ok(out)
    }

    fn parse_number(&mut self) -> Result<Json, String> {
        let start = self.pos;
        if self.peek() == Some('-') { self.next_char(); }
        while let Some(c) = self.peek() { if c.is_digit(10) { self.next_char(); } else { break } }
        if self.peek() == Some('.') { self.next_char(); while let Some(c) = self.peek() { if c.is_digit(10) { self.next_char(); } else { break } } }
        let substr = &self.s[start..self.pos];
        match substr.parse::<f64>() { Ok(n) => Ok(Json::Number(n)), Err(_) => Err("invalid number".into()) }
    }

    fn parse_array(&mut self) -> Result<Json, String> {
        self.next_char();
        self.skip_ws();
        let mut items = Vec::new();
        if self.peek() == Some(']') { self.next_char(); return Ok(Json::Array(items)); }
        loop {
            self.skip_ws();
            let v = self.parse_value()?;
            items.push(v);
            self.skip_ws();
            match self.peek() {
                Some(',') => { self.next_char(); continue; }
                Some(']') => { self.next_char(); break; }
                Some(c) => return Err(format!("unexpected in array: {}", c)),
                None => return Err("unterminated array".into()),
            }
        }
        Ok(Json::Array(items))
    }

    fn parse_object(&mut self) -> Result<Json, String> {
        self.next_char();
        self.skip_ws();
        let mut map = HashMap::new();
        if self.peek() == Some('}') { self.next_char(); return Ok(Json::Object(map)); }
        loop {
            self.skip_ws();
            let key = self.parse_string()?;
            self.skip_ws();
            if self.next_char() != Some(':') { return Err("expected ':'".into()); }
            self.skip_ws();
            let value = self.parse_value()?;
            map.insert(key, value);
            self.skip_ws();
            match self.peek() {
                Some(',') => { self.next_char(); continue; }
                Some('}') => { self.next_char(); break; }
                Some(c) => return Err(format!("unexpected in object: {}", c)),
                None => return Err("unterminated object".into()),
            }
        }
        Ok(Json::Object(map))
    }
}
