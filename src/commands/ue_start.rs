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
            .required("target2", SyntaxShape::OneOf(vec![
                SyntaxShape::Keyword(b"uproject".to_vec(), Box::new(SyntaxShape::String)), 
            ]), "Path to a uproject")
            // .switch("target", "Target to start", Some(vec![
            //     "editor".to_string(),
            //     "game".to_string(),
            //     "server".to_string(),
            // ]))
            .switch("shout", "(FIXME) Yell it instead", None)
            .category(Category::Experimental)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: "ue Ellie",
                description: "Say hello to Ellie",
                result: Some(Value::test_string("Hello, Ellie. How are you today?")),
            },
            Example {
                example: "ue --shout Ellie",
                description: "Shout hello to Ellie",
                result: Some(Value::test_string("HELLO, ELLIE. HOW ARE YOU TODAY?")),
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
        let mut greeting = format!("Hello, {name}. How are you today?");
        if call.has_flag("shout")? {
            greeting = greeting.to_uppercase();
        }
        Ok(Value::string(greeting, call.head))
    }
}

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    // This will automatically run the examples specified in your command and compare their actual
    // output against what was specified in the example.
    //
    // We recommend you add this test to any other commands you create, or remove it if the examples
    // can't be tested this way.

    PluginTest::new("unreal_engine", UnrealEnginePlugin.into())?
        .test_command_examples(&UEStart)
}
