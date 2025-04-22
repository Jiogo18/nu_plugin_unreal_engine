use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Signature, SyntaxShape, Value};

use crate::UnrealEnginePlugin;

pub struct UEStart;

// https://dev.epicgames.com/documentation/en-us/unreal-engine/unreal-engine-command-line-arguments-reference
// https://unrealcommunity.wiki/command-line-interface-cli-3mcqmc4z

impl SimplePluginCommand for UEStart {
    type Plugin = UnrealEnginePlugin;

    fn name(&self) -> &str {
        "ue start"
    }

    fn description(&self) -> &str {
        "(FIXME) help text for ue start"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional("uproject_path", SyntaxShape::Filepath, "Path to a uproject")
            .required(
                "target2",
                SyntaxShape::OneOf(vec![SyntaxShape::Keyword(
                    b"uproject".to_vec(),
                    Box::new(SyntaxShape::Nothing),
                )]),
                "Path to a uproject",
            )
            .required("mode", SyntaxShape::String, "either 'client' or 'server'")
            .switch("editor", "Target to start", Some('e'))
            .category(Category::Experimental)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: "ue start editor",
                description: "Start the editor in the current project",
                result: None,
            },
            Example {
                example: "ue start game",
                description: "Start a client game in the current project",
                result: None,
            },
            Example {
                example: "ue start server",
                description: "Start a server in the current project",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        _plugin: &UnrealEnginePlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let name: String = call.req(0)?;
        let greeting = format!("Hello, {name}. How are you today?");
        Ok(Value::string(greeting, call.head))
    }
}
