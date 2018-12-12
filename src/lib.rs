use std::str::FromStr;

#[derive(Debug)]
pub enum CellType {
    Int(i32),
    Text(String),
    Empty,
}

#[derive(Debug,PartialEq)]
pub enum Command {
    Set((char, i32), String),
    Get((char, i32)),
    Exit,
    Empty,
}

impl Command {
    fn get_args<'a, T: Iterator<Item=&'a str>>(mut iter: T) -> Result<Self, CommandParseError> {
        match iter.next() {
            Some(x) => {
                let pieces: Vec<_> = x.split(':').collect();

                if pieces.len() != 2 {
                    return Err(CommandParseError::InvalidCellSpecifier);
                }

                let chars: Vec<_> = pieces[0].chars().collect();

                if chars.len() != 1 {
                    return Err(CommandParseError::InvalidCellSpecifier);
                }

                let number_spec = pieces[1].parse();

                match (chars[0], number_spec) {
                    (letter @ 'A'...'Z', Ok(number)) => {
                        Ok(Command::Get((letter, number)))
                    },
                    _ => {
                        return Err(CommandParseError::InvalidCellSpecifier);
                    }
                }
            },
            None => Err(CommandParseError::MissingArguments),
        }
    }

    fn set_args<T: Iterator>(iter: T) -> Result<Self, CommandParseError> {
        unimplemented!();
    }
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut pieces = source.split_whitespace();

        match pieces.next() {
            Some(s) => {
                match s {
                    "get" => Self::get_args(pieces),
                    "set" => Self::set_args(pieces),
                    _ => Err(CommandParseError::InvalidCommand),
                }
            },
            None => Ok(Command::Empty),
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum CommandParseError {
    InvalidCommand,
    InvalidArguments,
    InvalidCellSpecifier,
    MissingArguments,
}

pub fn display_error(error: CommandParseError) -> &'static str {
    match error {
        CommandParseError::InvalidCommand => "Comando inválido",
        CommandParseError::InvalidArguments => "Argumentos inválidos",
        CommandParseError::InvalidCellSpecifier => "La casilla dada no existe",
        CommandParseError::MissingArguments => "Faltan argumentos",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_command() {
        assert_eq!("".parse(), Ok(Command::Empty));
    }

    #[test]
    fn set_command() {
        assert_eq!(
            "set A:15 40".parse(),
            Ok(Command::Set(('A', 15), "40".to_owned()))
        );
    }

    #[test]
    fn get_command() {
        assert_eq!(
            "get B:15".parse(),
            Ok(Command::Get(('B', 15)))
        );
    }
}
