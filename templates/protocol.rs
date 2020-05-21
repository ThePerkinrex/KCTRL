#[derive(Clone, Debug)]
pub enum Value {
    {{ enum_elements}}
}
impl Value {
    pub fn repr(&self) -> Vec<u8> {
        let mut res = Vec::new();
        match self {
            {{repr_match_contents}}
        };
        return res;
    }
}

#[derive(Clone, Debug)]
pub struct ProtoParser {
    tokens: Vec<u8>,
    kind: usize,
}

impl ProtoParser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            kind: usize::MAX,
        }
    }

    fn inner_parse(&mut self, byte: u8) -> Option<Value> {
        if self.tokens.len() == 0 {
            match byte as usize {
                {{ parser_token_new }}
                _ => self.kind = usize::MAX,
            };
            if self.kind != usize::MAX {
                self.tokens.push(byte);
            } // else ignore it, wait for a valid value
        } else if self.kind != usize::MAX {
            self.tokens.push(byte);
            match self.kind {
                {{ parser_token_end }}
                _ => self.kind = usize::MAX,
            }
        }
        None
    }

    pub fn parse(&mut self, byte: u8) -> Option<Value> {
        let r = self.inner_parse(byte);
        if let Some(_) = r {
            self.tokens.clear();
            self.kind = usize::MAX;
        }
        return r
    }
}
{{consts}}