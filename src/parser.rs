pub struct Parser {
   pub args: Vec<String>,
   pub sub_commands: Vec<SubCommand>,
}

pub struct SubCommand {
    pub name: String,
    pub help: String,
    pub description: String,
}