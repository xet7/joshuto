use crate::commands::{self, JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::textfield::TextField;
use crate::ui;
use crate::ui::TuiBackend;

#[derive(Clone, Debug)]
pub struct CommandLine {
    pub prefix: String,
    pub suffix: String,
}

impl CommandLine {
    pub fn new(prefix: String, suffix: String) -> Self {
        CommandLine { prefix, suffix }
    }
    pub const fn command() -> &'static str {
        "console"
    }

    pub fn readline(
        &self,
        context: &mut JoshutoContext,
        backend: &mut TuiBackend,
    ) -> JoshutoResult<()> {
        //        let mut textfield = TextField::new(&mut terminal, &context.events);
        let user_input: Option<String> = None; //textfield.readline();

        if let Some(s) = user_input {
            let trimmed = s.trim_start();
            match trimmed.find(' ') {
                Some(ind) => {
                    let (command, xs) = trimmed.split_at(ind);
                    let xs = xs.trim_start();
                    let args: Vec<String> = vec![String::from(xs)];
                    commands::from_args(String::from(command), args)?.execute(context, backend)
                }
                None => commands::from_args(String::from(trimmed), Vec::new())?
                    .execute(context, backend),
            }
        } else {
            Ok(())
        }
    }
}

impl JoshutoCommand for CommandLine {}

impl std::fmt::Display for CommandLine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {} {}", Self::command(), self.prefix, self.suffix)
    }
}

impl JoshutoRunnable for CommandLine {
    fn execute(&self, context: &mut JoshutoContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
        let res = self.readline(context, backend);
        ncurses::doupdate();
        res
    }
}
