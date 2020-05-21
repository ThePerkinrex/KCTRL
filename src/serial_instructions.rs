use crate::protocol;

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

    pub fn parse(&mut self, byte: u8) -> Option<protocol::Value> {
        if self.tokens.len() == 0 {
            match byte as usize {
                protocol::ANALOG_VAL_ID => self.kind = protocol::ANALOG_VAL_ID,
                protocol::DIGITAL_VAL_ID => self.kind = protocol::DIGITAL_VAL_ID,
                protocol::LCDWRITE_VAL_ID => self.kind = protocol::LCDWRITE_VAL_ID,
                _ => self.kind = usize::MAX,
            };
            if self.kind != usize::MAX {
                self.tokens.push(byte);
            } // else ignore it, wait for a valid value
        } else if self.kind != usize::MAX {
            self.tokens.push(byte);
            match self.kind {
                protocol::ANALOG_VAL_ID => {
                    if self.tokens.len() == protocol::ANALOG_VAL_LEN {
                        println!("ANALOG_VAL");
                    }
                }
                protocol::DIGITAL_VAL_ID => {
                    if self.tokens.len() == protocol::DIGITAL_VAL_LEN {
                        println!("DIGITAL_VAL");
                    }
                }
                protocol::LCDWRITE_VAL_ID => {
                    if byte == 255 {
                        let mut prev_byte = false;
                        let mut i = 0;
                        for tok in self.tokens.iter() {
                            i += 1;
                            if *tok == 255 && i != self.tokens.len() {
                                prev_byte = true;
                                break;
                            }
                        }
                        if prev_byte {
                            self.tokens.reverse();
                            self.tokens.pop();
                            
                            let mut l1 = String::new();
                            loop {
                                if let Some(tok) = self.tokens.pop() {
                                    if tok == 255 {
                                        break;
                                    } else {
                                        l1.push(tok as char);
                                    }
                                }
                            }
                            let mut l2 = String::new();
                            loop {
                                if let Some(tok) = self.tokens.pop() {
                                    if tok == 255 {
                                        break;
                                    } else {
                                        l2.push(tok as char);
                                    }
                                }

                            }
                            return Some(protocol::Value::Lcdwrite(l1, l2));
                        }
                    }
                },
                _ => self.kind = usize::MAX,
            }
        }
        None
    }
}
