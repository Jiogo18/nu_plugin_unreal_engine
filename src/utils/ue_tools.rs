use std::process::{Command, Stdio};

use nu_protocol::{LabeledError, PipelineData, Span, Value};

pub fn run(command: &mut Command, span: Span) -> Result<PipelineData, LabeledError> {
    match command
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(child) => match child.wait_with_output() {
            Ok(output) => Ok(PipelineData::Value(
                Value::String {
                    val: String::from_utf8_lossy(&output.stdout).to_string(),
                    internal_span: span,
                },
                None,
            )),
            Err(e) => Err(LabeledError::new(format!(
                "Failed to wait for command: {}",
                e.to_string()
            ))),
        },
        Err(e) => Err(LabeledError::new(format!(
            "Failed to spawn command: {}",
            e.to_string()
        ))),
    }
}
