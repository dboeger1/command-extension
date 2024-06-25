mod error;


use std::{
    process::{
        Command,
        Output,
    },
    str::from_utf8,
};

pub use error::Error;


pub struct ExecuteParameters {
    pub print_on_success: bool,
}
pub struct ExecuteResult {
    pub output: Output,
}

pub trait CommandExtension {
    fn execute(&mut self, parameters: &ExecuteParameters)
        -> Result<ExecuteResult, Error>;
}

impl CommandExtension for Command {
    fn execute(&mut self, parameters: &ExecuteParameters)
        -> Result<ExecuteResult, Error> {
        // Assemble command string.
        let mut command_string = self
            .get_program()
            .to_os_string();
        self
            .get_args()
            .for_each(|arg| command_string.push(format!(
                " {}",
                arg.to_string_lossy(),
            )));

        // Run command.
        let command_output = self
            .output()
            .map_err(|error| Error {
                message: format!(
                    "Failed to execute command: {}",
                    command_string.to_string_lossy(),
                ),
                source: Some(Box::new(error)),
            })?;
        let command_success = command_output.status.success();

        // Print command output.
        if !command_success || parameters.print_on_success {
            // Select between standard and error output.
            let print = |message: &str| match command_success {
                true => println!("{message}"),
                false => eprintln!("{message}"),
            };

            // Print command string.
            print("==== Command ====");
            print(format!(
                "{}",
                command_string.to_string_lossy(),
            ).as_str());

            // Print command exit code.
            let command_exit_code = command_output
                .status
                .code();
            print("==== Exit Code ====");
            print(format!(
                "{}",
                command_exit_code.map_or_else(
                    || "<failed to retrieve>".to_string(),
                    |status| status.to_string(),
                )
            ).as_str());

            // Print command stdout.
            let command_stdout = from_utf8(&command_output.stdout);
            print("==== stdout ====");
            if command_stdout.is_ok() {
                print(format!(
                    "{}",
                    command_stdout.unwrap_or("<failed to retrieve>"),
                ).as_str());
            }

            // Print command stderr.
            let command_stderr = from_utf8(&command_output.stderr);
            print("==== stderr ====");
            if command_stderr.is_ok() {
                print(format!(
                    "{}",
                    command_stderr.unwrap_or("<failed to retrieve>"),
                ).as_str());
            }
        }

        Ok(ExecuteResult {
            output: command_output,
        })
    }
}
