/// Represents a script.
#[derive(Debug, Clone)]
pub struct Script {
    cmds: Vec<Vec<u8>>,
}

impl Script {
    /// Creates a new `Script` instance.
    ///
    /// # Arguments
    ///
    /// * `commands` - Optional vector of commands (opcodes or data) to initialize the script with.
    ///
    /// # Returns
    ///
    /// A new `Script` instance.
    pub fn new(commands: Option<Vec<Vec<u8>>>) -> Script {
        match commands {
            Some(cmds) => Script { cmds },
            None => Script { cmds: Vec::new() },
        }
    }

    /// Converts the `Script` instance into a byte representation.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the byte representation of the `Script`.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        for cmd in &self.cmds {
            let length = cmd.len();
            if length == 1 {
                buffer.push(cmd[0]);
            } else {
                if length <= 75 {
                    buffer.push(length as u8);
                } else if length <= 0xff {
                    buffer.push(76);
                    buffer.push(length as u8);
                } else if length <= 520 {
                    buffer.push(77);
                    let length = (length as u16).to_le_bytes();
                    buffer.extend(length);
                }
                buffer.extend(cmd);
            }
        }

        buffer
    }
}
