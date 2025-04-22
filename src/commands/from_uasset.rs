use std::{io::Cursor, num::NonZero};

use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, Example, LabeledError, PipelineData, Signature, Span, Value, record};
use uasset::{AssetHeader, NameReference, ObjectImportOuter};

use crate::UnrealEnginePlugin;

pub struct FromUAsset;

fn name_reference_serializable(
    name_reference: NameReference,
    span: Span,
    replace_names: bool,
    names: &[String],
) -> Value {
    if replace_names {
        Value::string(names[name_reference.index as usize].clone(), span)
    } else {
        Value::record(
            record! {
                "index" => Value::int(name_reference.index as i64, span),
                "number" => name_reference.number.map(|n| Value::int((n as NonZero<u32>).get() as i64,span)).unwrap_or_default(),
            },
            span,
        )
    }
}

fn convert_asset_header_to_value<R: std::io::Read>(
    header: &AssetHeader<R>,
    span: Span,
    replace_names: bool,
) -> Value {
    Value::record(
        record! {
            "archive" => Value::record(record! {
                "file_version" => Value::int(header.archive.file_version as i64, span),
                "file_version_ue5" => header.archive.file_version_ue5.map(|f| Value::int(f as i64, span)).unwrap_or_default(),
                "file_licensee_version" => Value::int(header.archive.file_licensee_version as i64, span),
                "legacy_version" => Value::int(header.archive.legacy_version as i64, span),
                "with_editoronly_data" => Value::bool(header.archive.with_editoronly_data, span),
            }, span),
            "total_header_size" => Value::int(header.total_header_size as i64, span),
            "folder_name" => Value::string(header.folder_name.clone(), span),
            "package_flags" => Value::int(header.package_flags as i64, span),
            "names" => Value::list(header.names.iter().map(|n| Value::string(n, span)).collect::<Vec<_>>(), span),
            "soft_object_paths_count" => Value::int(header.soft_object_paths_count as i64, span),
            "soft_object_paths_offset" => Value::int(header.soft_object_paths_offset as i64, span),
            "localization_id" => header.localization_id.clone().map(|s| Value::string(s, span)).unwrap_or_default(),
            "gatherable_text_data_count" => Value::int(header.gatherable_text_data_count as i64, span),
            "gatherable_text_data_offset" => Value::int(header.gatherable_text_data_offset as i64, span),
            "export_count" => Value::int(header.export_count as i64, span),
            "export_offset" => Value::int(header.export_offset as i64, span),
            "imports" => Value::list(header.imports.iter().map(|i|
                Value::record(record!{
                "class_package" => name_reference_serializable(i.class_package, span, replace_names, &header.names),
                "class_name" => name_reference_serializable(i.class_name, span, replace_names, &header.names),
                "outer_index" => Value::int(match i.outer() {
                    // Reverse https://github.com/jorgenpt/uasset-rs/blob/8b6ecff0c0b19c2a7e0556375742ed90095ff881/src/lib.rs#L171-L178
                    // Same as https://github.com/EpicGames/UnrealEngine/blob/2d53fcab0066b1f16dd956b227720841cad0f6f7/Engine/Source/Runtime/CoreUObject/Public/UObject/ObjectResource.h#L90-L101
                    ObjectImportOuter::Root => 0,
                    ObjectImportOuter::Import { import_index } => -(import_index as i64) - 1,
                    ObjectImportOuter::Export { export_index } => (export_index as i64) + 1,
                }, span),
                "object_name" => name_reference_serializable(i.object_name, span, replace_names, &header.names),
                "package_name" => i.package_name.map(|n| name_reference_serializable(n, span, replace_names, &header.names)).unwrap_or_default(),
                "import_optional" => Value::bool(i.import_optional, span),
            }, span)).collect::<Vec<_>>(), span),

            "depends_offset" => Value::int(header.depends_offset as i64, span),
            "soft_package_references_count" => Value::int(header.soft_package_references_count as i64, span),
            "soft_package_references_offset" => Value::int(header.soft_package_references_offset as i64, span),
            "searchable_names_offset" => header.searchable_names_offset.map(|n| Value::int(n as i64, span)).unwrap_or_default(),
            "thumbnail_table_offset" => Value::int(header.thumbnail_table_offset as i64, span),
            "engine_version" => Value::record(record!{
                "major" => Value::int(header.engine_version.major as i64, span),
                "minor" => Value::int(header.engine_version.minor as i64, span),
                "patch" => Value::int(header.engine_version.patch as i64, span),
                "changelist" => Value::int(header.engine_version.changelist as i64, span),
                "is_licensee_version" => Value::bool(header.engine_version.is_licensee_version, span),
                "branch_name" => Value::string(header.engine_version.branch_name.clone(), span),
            }, span),
            "compatible_with_engine_version" => Value::record(record!{
                "major" => Value::int(header.compatible_with_engine_version.major as i64, span),
                "minor" => Value::int(header.compatible_with_engine_version.minor as i64, span),
                "patch" => Value::int(header.compatible_with_engine_version.patch as i64, span),
                "changelist" => Value::int(header.compatible_with_engine_version.changelist as i64, span),
                "is_licensee_version" => Value::bool(header.compatible_with_engine_version.is_licensee_version, span),
                "branch_name" => Value::string(header.compatible_with_engine_version.branch_name.clone(), span),
            }, span),
            "compression_flags" => Value::int(header.compression_flags as i64, span),
            "package_source" => Value::int(header.package_source as i64, span),
            "additional_packages_to_cook" => Value::list(header.additional_packages_to_cook.iter().map(|n| Value::string(n, span)).collect::<Vec<_>>(), span),
            "texture_allocations" => header.texture_allocations.map(|n| Value::int(n as i64, span)).unwrap_or_default(),
            "asset_registry_data_offset" => Value::int(header.asset_registry_data_offset as i64, span),
            "bulk_data_start_offset" => Value::int(header.bulk_data_start_offset as i64, span),
            "world_tile_info_data_offset" => header.world_tile_info_data_offset.map(|n| Value::int(n as i64, span)).unwrap_or_default(),
            "chunk_ids" => Value::list(header.chunk_ids.iter().map(|n| Value::int(*n as i64, span)).collect::<Vec<_>>(), span),
            "preload_dependency_count" => Value::int(header.preload_dependency_count as i64, span),
            "preload_dependency_offset" => Value::int(header.preload_dependency_offset as i64, span),
            "names_referenced_from_export_data_count" => Value::int(header.names_referenced_from_export_data_count as i64, span),
            "payload_toc_offset" => Value::int(header.payload_toc_offset as i64, span),
            "data_resource_offset" => header.data_resource_offset.map(|n| Value::int(n as i64, span)).unwrap_or_default(),
        },
        span,
    )
}

fn from_uasset_header(
    bytes: &[u8],
    span: Span,
    replace_names: bool,
) -> Result<Value, uasset::Error> {
    let cursor = Cursor::new(bytes);
    let asset_header = AssetHeader::new(cursor)?;
    let value = convert_asset_header_to_value(&asset_header, span, replace_names);
    Ok(value)
}

impl PluginCommand for FromUAsset {
    type Plugin = UnrealEnginePlugin;

    fn name(&self) -> &str {
        "from uasset"
    }

    fn description(&self) -> &str {
        "Open a uasset file and return its header as a record"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .switch(
                "replace-names",
                "Replace index by the conrresponding name",
                None,
            )
            .category(Category::Conversions)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: "open BP_ThirdPersonCharacter.uasset",
                description: "Open a uasset file and return its header as a record",
                result: None,
            },
            Example {
                example: "open BP_ThirdPersonCharacter.uasset --raw | from uasset --replace-names",
                description: "Open a uasset file and return its header as a record",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        _plugin: &UnrealEnginePlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let replace_names = call.has_flag("replace-names")?;
        match input {
            PipelineData::Empty => Ok(PipelineData::Empty),
            PipelineData::Value(v, meta) => match v {
                Value::Binary {
                    val,
                    internal_span: span,
                } => {
                    let value = from_uasset_header(&val, span, replace_names)
                        .map_err(|e| LabeledError::new(e.to_string()))?;
                    Ok(PipelineData::Value(value, meta))
                }
                v => Err(LabeledError::new(format!(
                    "requires binary input, got {}",
                    v.get_type()
                ))),
            },
            PipelineData::ListStream(_, _) => Err(LabeledError::new("unsupported list stream")),
            PipelineData::ByteStream(stream, meta) => {
                let value = stream.into_value()?;
                match value {
                    Value::Binary {
                        val,
                        internal_span: span,
                    } => {
                        let value = from_uasset_header(&val, span, replace_names)
                            .map_err(|e| LabeledError::new(e.to_string()))?;
                        Ok(PipelineData::Value(value, meta))
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    // Test the format and arguments of the examples.
    // The output would be too complex, but it is not verified since we use None.

    PluginTest::new("unreal_engine", UnrealEnginePlugin.into())?.test_command_examples(&FromUAsset)
}
