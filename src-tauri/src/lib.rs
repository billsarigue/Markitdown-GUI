use serde::{Deserialize, Serialize};
use tauri_plugin_shell::ShellExt;

// ── Tipos de comunicação com o sidecar ──────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum SidecarResponse {
    Success {
        success: bool,
        content: Option<String>,
        output_path: Option<String>,
    },
    Failure {
        success: bool,
        error: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertOptions {
    pub input_path: String,
    pub output_path: Option<String>,
    pub llm_endpoint: Option<String>,
    pub llm_deployment: Option<String>,
    pub llm_api_key: Option<String>,
    pub llm_model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertResult {
    pub success: bool,
    pub content: Option<String>,
    pub output_path: Option<String>,
    pub error: Option<String>,
}

// ── Comando principal ────────────────────────────────────────────────────────

#[tauri::command]
async fn convert_file(
    app: tauri::AppHandle,
    options: ConvertOptions,
) -> Result<ConvertResult, String> {
    let mut args: Vec<String> = vec![options.input_path.clone()];

    if let Some(ref out) = options.output_path {
        args.push(out.clone());
    }

    if let (Some(endpoint), Some(deployment), Some(api_key)) = (
        options.llm_endpoint.clone(),
        options.llm_deployment.clone(),
        options.llm_api_key.clone(),
    ) {
        args.push("--llm-endpoint".to_string());
        args.push(endpoint);
        args.push("--llm-deployment".to_string());
        args.push(deployment);
        args.push("--llm-api-key".to_string());
        args.push(api_key);
    } else if let Some(ref api_key) = options.llm_api_key {
        args.push("--llm-api-key".to_string());
        args.push(api_key.clone());
        if let Some(ref model) = options.llm_model {
            args.push("--llm-model".to_string());
            args.push(model.clone());
        }
    }

    let shell = app.shell();
    let sidecar = shell
        .sidecar("markitdown")
        .map_err(|e| format!("Sidecar not found: {e}"))?;

    let output = sidecar
        .args(&args)
        .output()
        .await
        .map_err(|e| format!("Sidecar execution failed: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Ok(ConvertResult {
            success: false,
            content: None,
            output_path: None,
            error: Some(format!("Sidecar exited with error: {stderr}")),
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let response: SidecarResponse = serde_json::from_str(stdout.trim())
        .map_err(|e| format!("JSON parse error: {e}. Raw: {stdout}"))?;

    match response {
        SidecarResponse::Success { success, content, output_path } => Ok(ConvertResult {
            success,
            content,
            output_path,
            error: None,
        }),
        SidecarResponse::Failure { error, .. } => Ok(ConvertResult {
            success: false,
            content: None,
            output_path: None,
            error: Some(error),
        }),
    }
}

// ── Comando auxiliar: diálogo de arquivo nativo ──────────────────────────────

#[tauri::command]
async fn pick_files(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let files = app
        .dialog()
        .file()
        .add_filter(
            "Supported files",
            &[
                "pdf", "docx", "doc", "pptx", "ppt", "xlsx", "xls",
                "html", "htm", "txt", "csv", "json", "xml",
                "jpg", "jpeg", "png", "gif", "bmp", "webp",
                "zip", "epub",
            ],
        )
        .blocking_pick_files();

    match files {
        Some(paths) => Ok(paths
            .iter()
            .filter_map(|p| p.as_path().map(|p| p.to_string_lossy().to_string()))
            .collect()),
        None => Ok(vec![]),
    }
}

// ── Entry point ──────────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![convert_file, pick_files])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
