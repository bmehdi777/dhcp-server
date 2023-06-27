use std::fmt;

#[derive(Debug, Clone)]
pub struct OptionField {
    pub magic_cookies: [u8; 4],
    pub options: Vec<OptionSubfield>,
}
#[derive(Debug, Clone)]
pub struct OptionSubfield {
    pub op_code: u8,
    pub op_len: u8,
    pub data: Vec<u8>,
}

impl OptionSubfield {
    pub fn new(op_code: u8, op_len: u8, data: Vec<u8>) -> OptionSubfield {
        OptionSubfield {
            op_code,
            op_len,
            data,
        }
    }
    pub fn from_bytes(input: Vec<u8>) -> OptionSubfield {
        OptionSubfield {
            op_code: input[0],
            op_len: input[1],
            data: input[2..input.len()].to_vec(),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![self.op_code, self.op_len];
        bytes.extend(&self.data);
        bytes
    }
}
impl OptionField {
    pub fn new(
        options: Vec<OptionSubfield>,
    ) -> OptionField {
        OptionField {
            magic_cookies: [99,130,83,99], // set by rfc 1497
            options,
        }
    }
    pub fn from_bytes(input: Vec<u8>) -> OptionField {
        let magic_cookies: [u8; 4] = input[0..=3]
            .try_into()
            .expect("slice with incorrect length");
        let mut offset = 4;
        let mut options: Vec<OptionSubfield> = Vec::new();

        while offset < input.len() - 1 {
            match input[offset] {
                255 => {
                    break;
                }
                0 => {
                    // pad
                    offset += 1;
                }
                _ => {
                    let length: usize = input[offset + 1].into();
                    options.push(OptionSubfield::from_bytes(
                        input[offset..=offset + 1 + length].to_vec(),
                    ));
                    offset += length + 2;
                }
            }
        }

        OptionField {
            magic_cookies,
            options,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.magic_cookies.to_vec();
        for op in self.options.iter() {
            bytes.extend(&op.to_bytes());
        }
        bytes.push(255); // termination bytes
        bytes
    }
}

impl fmt::Display for OptionSubfield {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "code: {} len: {} data: {:X?}",
            self.op_code, self.op_len, self.data
        )
    }
}

impl fmt::Display for OptionField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res: String = String::from(format!(
            "{{ magic_cookies : {:X?} subfield: [ ",
            self.magic_cookies
        ));
        for op in self.options.iter() {
            res = res + &format!("{{ {} }},", op);
        }
        write!(f, "{}] }}", res)
    }
}
