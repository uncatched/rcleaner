use std::process::exit;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Info(InfoSubCommand),
    Clean(CleanSubCommand),
}

#[derive(Debug, PartialEq, Eq)]
pub enum InfoSubCommand {
    All,
    Caches,
    DerivedData,
    Simulators,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CleanSubCommand {
    All,
    Caches,
    DerivedData,
    Simulators(String),
}

impl Command {
    pub fn new(args: &Vec<String>) -> Self {
        let cmd_raw = &args[1];
        match &cmd_raw[..] {
            "info" => {
                let sub_cmd_raw = &args[2];
                let info_sub_cmd = match &sub_cmd_raw[..] {
                    "all" => InfoSubCommand::All,
                    "caches" => InfoSubCommand::Caches,
                    "derived-data" => InfoSubCommand::DerivedData,
                    "simulators" => InfoSubCommand::Simulators,
                    _ => panic!("Unexpected command"),
                };

                Command::Info(info_sub_cmd)
            }
            "clean" => {
                let sub_cmd_raw = &args[2];
                let clean_sub_cmd = match &sub_cmd_raw[..] {
                    "all" => CleanSubCommand::All,
                    "caches" => CleanSubCommand::Caches,
                    "derived-data" => CleanSubCommand::DerivedData,
                    "simulators" => {
                        let udid = &args[3];
                        CleanSubCommand::Simulators(udid.to_owned())
                    }
                    _ => panic!("Unexpected command"),
                };

                Command::Clean(clean_sub_cmd)
            }
            _ => panic!("Unexpected command"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_info_all_cmd() {
        let args: Vec<String> = vec!["".to_string(), "info".to_string(), "all".to_string()];
        let cmd = Command::new(&args);

        assert_eq!(cmd, Command::Info(InfoSubCommand::All))
    }

    #[test]
    fn test_new_info_caches_cmd() {
        let args: Vec<String> = vec!["".to_string(), "info".to_string(), "caches".to_string()];
        let cmd = Command::new(&args);

        assert_eq!(cmd, Command::Info(InfoSubCommand::Caches))
    }

    #[test]
    fn test_new_info_derived_data_cmd() {
        let args: Vec<String> = vec![
            "".to_string(),
            "info".to_string(),
            "derived-data".to_string(),
        ];
        let cmd = Command::new(&args);

        assert_eq!(cmd, Command::Info(InfoSubCommand::DerivedData))
    }

    #[test]
    fn test_new_info_simulators_cmd() {
        let args: Vec<String> = vec!["".to_string(), "info".to_string(), "simulators".to_string()];
        let cmd = Command::new(&args);

        assert_eq!(cmd, Command::Info(InfoSubCommand::Simulators))
    }

    #[test]
    fn test_new_clean_all_cmd() {
        let args: Vec<String> = vec!["".to_string(), "clean".to_string(), "all".to_string()];
        let cmd = Command::new(&args);

        assert_eq!(cmd, Command::Clean(CleanSubCommand::All))
    }

    #[test]
    fn test_new_clean_caches_cmd() {
        let args: Vec<String> = vec!["".to_string(), "clean".to_string(), "caches".to_string()];
        let cmd = Command::new(&args);

        assert_eq!(cmd, Command::Clean(CleanSubCommand::Caches))
    }

    #[test]
    fn test_new_clean_derived_data_cmd() {
        let args: Vec<String> = vec![
            "".to_string(),
            "clean".to_string(),
            "derived-data".to_string(),
        ];
        let cmd = Command::new(&args);

        assert_eq!(cmd, Command::Clean(CleanSubCommand::DerivedData))
    }

    #[test]
    fn test_new_clean_simulators_cmd() {
        let args: Vec<String> = vec![
            "".to_string(),
            "clean".to_string(),
            "simulators".to_string(),
            "1234".to_string(),
        ];
        let cmd = Command::new(&args);

        assert_eq!(
            cmd,
            Command::Clean(CleanSubCommand::Simulators(String::from("1234")))
        )
    }
}
