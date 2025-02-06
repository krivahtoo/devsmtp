#[derive(Debug, Default, Clone)]
pub enum Command {
    #[default]
    None,
    Data,
    Quit,
    Helo(String),
    Ehlo(String),
    Help(String),
    Auth(String),
    MailFrom(String),
    Receipient(String),
    Unknown(String),
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        match value.split(' ').collect::<Vec<&str>>()[..] {
            ["HELO", v, ..] => Command::Helo(v.to_string()),
            ["EHLO", v, ..] => Command::Ehlo(v.to_string()),
            ["HELP", v] => Command::Help(v.to_string()),
            ["AUTH", v, ..] => Command::Auth(v.to_string()),
            ["MAIL", v] if v.starts_with("FROM:") => {
                Command::MailFrom(v.to_string().replace("FROM:", ""))
            }
            ["RCPT", v] if v.starts_with("TO:") => {
                Command::Receipient(v.to_string().replace("TO:", ""))
            }
            ["DATA"] => Command::Data,
            ["QUIT"] => Command::Quit,
            [""] => Command::None,
            _ => Command::Unknown(value),
        }
    }
}
