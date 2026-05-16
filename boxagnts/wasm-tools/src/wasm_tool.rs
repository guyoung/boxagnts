use async_trait::async_trait;
use serde_json::Value;

use boxagnts_tools::ToolResult;
use boxagnts_tools::{PermissionLevel, Tool, ToolContext};

pub struct WasmTool {
    name: String,
    wasm_file: String,
    description: String,
    permission_level: PermissionLevel,
    input_schema: Value,
}

impl WasmTool {
    pub fn new(
        name: String,
        wasm_file: String,
        description: String,
        permission_level: PermissionLevel,
        input_schema: Value,
    ) -> Self {
        Self {
            name,
            wasm_file,
            description,
            permission_level,
            input_schema,
        }
    }
}

#[async_trait]
impl Tool for WasmTool {
    fn name(&self) -> &'static str {
        Box::leak(self.name.clone().into_boxed_str())
    }

    fn description(&self) -> &'static str {
        Box::leak(self.description.clone().into_boxed_str())
    }

    fn permission_level(&self) -> PermissionLevel {
        self.permission_level
    }

    fn input_schema(&self) -> Value {
        self.input_schema.clone()
    }

    async fn execute(&self, input: Value, ctx: &ToolContext) -> ToolResult {
        let args = value_to_cli_args(input);

        let wasm_file = format!(
            "{}",
            ctx.get_app_extensions_dir().await
                .join("tools/")
                .join(&self.wasm_file)
                .display()
        );

        let work_dir = ctx.get_work_dir().await;

        let cache_dir = ctx.get_app_cache_dir().await;

        let allowed_outbound_hosts = ctx.get_allowed_outbound_hosts();

        let mut options = boxagnts_wasm_sandbox::run::RunOption::default();

        options.work_dir = Some(format!("{}", work_dir.display()));
        options.allowed_outbound_hosts = Some(allowed_outbound_hosts);
        options.wasm_cache_dir = Some(format!("{}", cache_dir.display()));

        println!(
            "wasm_tool execute wasm file: {} args: {:?}",
            wasm_file, args
        );

        let result =
            boxagnts_wasm_sandbox::run::execute(wasm_file.clone(), None, Some(args), options, None)
                .await;

        let tool_resul = match result {
            Ok((result, err)) => {
                let (result, _encoding, _had_errors) = decode_bytes(result);

                let (err, _encoding, _had_errors) = decode_bytes(err);

                if !err.is_empty() {
                    ToolResult {
                        is_error: true,
                        content: err,
                        metadata: None,
                    }
                } else {
                    let val: serde_json::Result<Value> = serde_json::from_str(&result);

                    match val {
                        Ok(obj) => {
                            if let Value::Object(map) = obj {
                                let is_error = if let Some(is_error) = map.get("error") {
                                    is_error.as_bool().unwrap_or(false)
                                } else {
                                    false
                                };
                                let content = if let Some(content) = map.get("content") {
                                    content.as_str().unwrap_or("").to_string()
                                } else {
                                    "".to_string()
                                };
                                let metadata = if let Some(metadata) = map.get("metadata") {
                                    Some(metadata.clone())
                                } else {
                                    None
                                };

                                ToolResult {
                                    is_error,
                                    content,
                                    metadata,
                                }
                            } else {
                                ToolResult {
                                    is_error: false,
                                    content: obj.to_string(),
                                    metadata: None,
                                }
                            }
                        }
                        Err(_) => ToolResult {
                            is_error: false,
                            content: result,
                            metadata: None,
                        },
                    }
                }
            }
            Err(err) => ToolResult {
                is_error: true,
                content: format!("{:?}", err),
                metadata: None,
            },
        };

        tool_resul
    }
}

fn decode_bytes(bytes: bytes::Bytes) -> (String, &'static str, bool) {
    let mut detector = chardetng::EncodingDetector::new(chardetng::Iso2022JpDetection::Allow);
    detector.feed(&bytes, true);
    let encoding = detector.guess(None, chardetng::Utf8Detection::Allow);

    let (cow, _, had_errors) = encoding.decode(&bytes);

    (cow.into_owned(), encoding.name(), had_errors)
}

fn value_to_cli_args(value: Value) -> Vec<String> {
    let positional = positional_fields();
    let mut result = vec![];

    if let Value::Object(map) = value.clone() {
        for (key, val) in map {
            // 跳过位置参数字段，后面单独处理
            if positional.contains(&key.as_str()) {
                continue;
            }
            // 下划线转连字符
            let flag = key.replace('_', "-");
            match val {
                Value::Null => {} // Option::None，跳过
                Value::Bool(true) => {
                    result.push(format!("--{}", flag));
                }
                Value::Bool(false) => {}
                Value::String(s) => {
                    result.push(format!("--{}", flag));
                    result.push(s);
                }
                other => {
                    result.push(format!("--{}", flag));
                    result.push(other.to_string());
                }
            }
        }
    }

    // 追加位置参数
    if let Value::Object(map) = value {
        for key in positional {
            if let Some(val) = map.get(key) {
                if let Value::String(s) = val {
                    result.push(s.clone());
                }
            }
        }
    }

    result
}

fn positional_fields() -> Vec<&'static str> {
    vec![]
}
