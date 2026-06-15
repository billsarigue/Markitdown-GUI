// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::Manager;
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
    pub enable_plugins: Option<bool>,
    pub llm_endpoint: Option<String>,
    pub llm_deployment: Option<String>,
    pub llm_api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertResult {
    pub success: bool,
    pub content: Option<String>,
    pub output_path: Option<String>,
    pub error: Option<String>,
}

// ── Comando Tauri principal ──────────────────────────────────────────────────

#[tauri::command]
async fn convert_file(
    app: tauri::AppHandle,
    options: ConvertOptions,
) -> Result<ConvertResult, String> {
    // Monta os argumentos para o sidecar
    let mut args: Vec<String> = vec![options.input_path.clone()];

    if let Some(ref out) = options.output_path {
        args.push(out.clone());
    }

    if options.enable_plugins.unwrap_or(false) {
        args.push("--enable-plugins".to_string());
    }

    if let (Some(endpoint), Some(deployment), Some(api_key)) = (
        options.llm_endpoint,
        options.llm_deployment,
        options.llm_api_key,
    ) {
        args.push("--llm-endpoint".to_string());
        args.push(endpoint);
        args.push("--llm-deployment".to_string());
        args.push(deployment);
        args.push("--llm-api-key".to_string());
        args.push(api_key);
    }

    // Invoca o sidecar
    let shell = app.shell();
    let output = shell
        .sidecar("markitdown")
        .map_err(|e| format!("Sidecar not found: {e}"))?;

    let output = output
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

    // Parseia o JSON retornado pelo wrapper Python
    let response: SidecarResponse =
        serde_json::from_str(stdout.trim()).map_err(|e| format!("JSON parse error: {e}. Raw: {stdout}"))?;

    match response {
        SidecarResponse::Success {
            success,
            content,
            output_path,
        } => Ok(ConvertResult {
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

// ── Comando auxiliar: abre diálogo de arquivo nativo ────────────────────────

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
        None => Ok(vec![]), // usuário cancelou
    }
}

// ── Entry point ──────────────────────────────────────────────────────────────

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![convert_file, pick_files])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
