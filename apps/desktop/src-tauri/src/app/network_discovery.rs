use crate::network_discovery;

#[tauri::command]
#[specta::specta]
pub async fn discover_network(
) -> Result<Vec<network_discovery::NetworkInstanceInfo>, String> {
    match network_discovery::discover_network().await {
        Ok(instances) => Ok(instances),
        Err(e) => Err(e.to_string()),
    }
}
