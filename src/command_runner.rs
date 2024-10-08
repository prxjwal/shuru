use std::process::{Command, ExitStatus};

use shuru::{
    config::{Config, TaskConfig},
    error::Error,
};

pub struct CommandRunner {
    config: Config,
}

impl CommandRunner {
    pub fn new(config: Config) -> Self {
        CommandRunner { config }
    }

    fn find_task(&self, name: &str) -> Result<&TaskConfig, Error> {
        self.config
            .tasks
            .iter()
            .find(|task| {
                task.name == name
                    || task
                        .aliases
                        .as_ref()
                        .map_or(false, |aliases| aliases.contains(&name.to_string()))
            })
            .ok_or_else(|| Error::CommandNotFound(name.to_string()))
    }

    pub fn run_command(&self, name: &str) -> Result<ExitStatus, Error> {
        let task = self.find_task(name)?;

        let env_path = self.config.versions.iter().try_fold(
            String::new(),
            |env_path, (versioned_command, version_info)| {
                let version_manager = versioned_command.get_version_manager(version_info);
                let binary_path = version_manager.install_and_get_binary_path()?;

                Ok::<_, Error>(format!("{}:{}", binary_path.to_string_lossy(), env_path))
            },
        )?;

        let final_env_path = format!("{}{}", env_path, std::env::var("PATH").unwrap_or_default());

        let status = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .env("PATH", final_env_path)
                .args(["/C", &task.command])
                .status()
                .map_err(|e| {
                    Error::CommandExecutionError(format!("Failed to execute command: {}", e))
                })?
        } else {
            let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

            Command::new(shell)
                .env("PATH", final_env_path)
                .arg("-c")
                .arg(&task.command)
                .status()
                .map_err(|e| {
                    Error::CommandExecutionError(format!("Failed to execute command: {}", e))
                })?
        };

        Ok(status)
    }

    pub fn run_default(&self) -> Result<ExitStatus, Error> {
        if let Some(task) = self
            .config
            .tasks
            .iter()
            .find(|task| task.default.unwrap_or(false))
        {
            return self.run_command(&task.name);
        }

        Err(Error::DefaultCommandNotFound)
    }
}
