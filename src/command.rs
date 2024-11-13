use core::{fmt, str::{from_utf8, Utf8Error}};

static EOL_CHAR: char = ';';

pub struct EOLReachedError {}
impl fmt::Display for EOLReachedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot append to buffer after command is complete")
    }
}

pub struct Command {
    buf: [u8; 256],
    current_buf_idx: usize,
    is_eol: bool,
}


impl Command {
    pub fn new() -> Self {
        Command {
            buf: [0u8; 256],
            current_buf_idx: 0,
            is_eol: false,
        }
    }

    pub fn add_char_buf(&mut self, char_buf: &[u8; 1]) -> Result<(), EOLReachedError> {
        if self.is_eol {
            return Err(EOLReachedError {});
        }
        let buf_utf8 = from_utf8(char_buf).unwrap();
        let char = buf_utf8.chars().next().unwrap();
        if char == EOL_CHAR {
            // str split treats last split different, add extra split char to avoid
            self.buf[self.current_buf_idx] = b' ';
            self.is_eol = true;
            return Ok(());
        }
        self.buf[self.current_buf_idx] = char_buf[0];
        self.current_buf_idx += 1;
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        self.is_eol
    }

    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        from_utf8(&self.buf)
    }
}
