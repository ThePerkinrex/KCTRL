#[derive(Clone, Debug)]
pub enum Value {
    	Lcdwrite(String,String,),
	Lcdclear(bool,),
	Led_1(bool,),
	Handshake(bool,),
	Recieved(bool,),

}
impl Value {
    pub fn repr(&self) -> Vec<u8> {
        let mut res = Vec::new();
        match self {
            Self::Lcdwrite(l1,l2) => {
res.push(LCDWRITE_VAL_ID as u8);
res.append(&mut l1.bytes().collect::<Vec<u8>>().clone());
res.push(255);
res.append(&mut l2.bytes().collect::<Vec<u8>>().clone());
res.push(255);
},
Self::Lcdclear(k) => {
res.push(LCDCLEAR_VAL_ID as u8);
res.push(0|((*k as u8)<<(0 as u8)));
},
Self::Led_1(state) => {
res.push(LED_1_VAL_ID as u8);
res.push(0|((*state as u8)<<(0 as u8)));
},
Self::Handshake(side) => {
res.push(HANDSHAKE_VAL_ID as u8);
res.push(0|((*side as u8)<<(0 as u8)));
},
Self::Recieved(side) => {
res.push(RECIEVED_VAL_ID as u8);
res.push(0|((*side as u8)<<(0 as u8)));
},

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
                LCDWRITE_VAL_ID => self.kind = LCDWRITE_VAL_ID,
LCDCLEAR_VAL_ID => self.kind = LCDCLEAR_VAL_ID,
LED_1_VAL_ID => self.kind = LED_1_VAL_ID,
HANDSHAKE_VAL_ID => self.kind = HANDSHAKE_VAL_ID,
RECIEVED_VAL_ID => self.kind = RECIEVED_VAL_ID,

                _ => self.kind = usize::MAX,
            };
            if self.kind != usize::MAX {
                self.tokens.push(byte);
            } // else ignore it, wait for a valid value
        } else if self.kind != usize::MAX {
            self.tokens.push(byte);
            match self.kind {
                LCDWRITE_VAL_ID => {
let mut done = false;
let mut byte_count = 1;
if byte_count + 1 >= self.tokens.len() { return None }
while self.tokens[byte_count] != 255 {
byte_count += 1;
if byte_count >= self.tokens.len() { return None }
}
byte_count += 1;
if byte_count + 1 >= self.tokens.len() { return None }
while self.tokens[byte_count] != 255 {
byte_count += 1;
if byte_count >= self.tokens.len() { return None }
}
byte_count += 1;
done = self.tokens.len() == byte_count;
if done {
let mut last_index = 1;
let mut l1 = String::new();
while self.tokens[last_index] != 255 {
l1.push(self.tokens[last_index] as char);
last_index += 1;
}
last_index += 1;
let mut l2 = String::new();
while self.tokens[last_index] != 255 {
l2.push(self.tokens[last_index] as char);
last_index += 1;
}
last_index += 1;
return Some(Value::Lcdwrite(l1,l2));
}},
LCDCLEAR_VAL_ID => {
let mut done = false;
done = self.tokens.len() == LCDCLEAR_VAL_LEN;
if done {
let mut last_index = 1;
let k = (self.tokens[last_index] & 1 << 0) != 0;
last_index += 1;
return Some(Value::Lcdclear(k));
}},
LED_1_VAL_ID => {
let mut done = false;
done = self.tokens.len() == LED_1_VAL_LEN;
if done {
let mut last_index = 1;
let state = (self.tokens[last_index] & 1 << 0) != 0;
last_index += 1;
return Some(Value::Led_1(state));
}},
HANDSHAKE_VAL_ID => {
let mut done = false;
done = self.tokens.len() == HANDSHAKE_VAL_LEN;
if done {
let mut last_index = 1;
let side = (self.tokens[last_index] & 1 << 0) != 0;
last_index += 1;
return Some(Value::Handshake(side));
}},
RECIEVED_VAL_ID => {
let mut done = false;
done = self.tokens.len() == RECIEVED_VAL_LEN;
if done {
let mut last_index = 1;
let side = (self.tokens[last_index] & 1 << 0) != 0;
last_index += 1;
return Some(Value::Recieved(side));
}},

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
pub const LCDWRITE_VAL_ID: usize = 0;
pub const LCDCLEAR_VAL_ID: usize = 1;
pub const LCDCLEAR_VAL_LEN: usize = 2;
pub const LED_1_VAL_ID: usize = 2;
pub const LED_1_VAL_LEN: usize = 2;
pub const HANDSHAKE_VAL_ID: usize = 3;
pub const HANDSHAKE_VAL_LEN: usize = 2;
pub const RECIEVED_VAL_ID: usize = 4;
pub const RECIEVED_VAL_LEN: usize = 2;

