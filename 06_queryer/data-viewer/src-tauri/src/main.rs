#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn example_sql() -> String {
  queryer::example_sql()
}

#[tauri::command]
async fn query(sql: String) -> Result<String, String> {
  let data = queryer::query(&sql).await.map_err(|err| err.to_string())?;
  Ok(data.to_csv().map_err(|err| err.to_string())?)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![example_sql, query])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
