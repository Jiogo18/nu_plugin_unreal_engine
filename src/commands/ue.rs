use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Signature, SyntaxShape, Value, record};

use crate::{UnrealEnginePlugin, utils::uproject};

pub struct UE;

impl SimplePluginCommand for UE {
    type Plugin = UnrealEnginePlugin;

    fn name(&self) -> &str {
        "ue"
    }

    fn description(&self) -> &str {
        "Get informations about an Unreal Engine project"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .named(
                "uproject",
                SyntaxShape::Filepath,
                "Path to a uproject, default is the .uproject file of the current directory",
                Some('u'),
            )
            .category(Category::Experimental)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            example: "ue",
            description: "Get informations about the project",
            result: None,
        }]
    }

    fn run(
        &self,
        _plugin: &UnrealEnginePlugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let uproject_path =
            uproject::uproject_from_arg_or_current_dir(&engine, call.get_flag("uproject")?)?;
        let uproject = uproject::UProject::from_path(&uproject_path)?;

        // Output as a record
        let mut record = record! {
            "uproject_path" => Value::string(uproject.uproject_path.display().to_string(), call.head),
            "name" => Value::string(uproject.name, call.head),
            "ide" => Value::string(uproject.ide.to_string(), call.head),
        };
        if let Some(path) = uproject.unreal_engine_path {
            record.insert(
                "unreal_engine_path",
                Value::string(path.display().to_string(), call.head),
            );
        }
        Ok(Value::record(record, call.head))
    }
}
