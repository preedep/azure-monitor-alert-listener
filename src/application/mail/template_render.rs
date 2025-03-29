use tera::{Context, Tera};
use crate::domain::models::AzureMonitorAlert;

pub fn render_alert_email(
    template_dir: &str,
    template_name: &str,
    alert: &AzureMonitorAlert,
) -> Result<String, Box<dyn std::error::Error>> {
    // Load template
    let tera = Tera::new(template_dir)?;

    let essentials = &alert.data.essentials;
    let alert_context = &alert.data.alert_context;

    // Extract fields
    let resource_group = &essentials.target_resource_group;
    let default_app_name = String::from("-");
    let app_name = essentials
        .configuration_items
        .get(0)
        .unwrap_or(&default_app_name);
    let default_adf_name = String::from("-");
    let adf_name = essentials
        .alert_target_ids
        .get(0)
        .unwrap_or(&default_adf_name)
        .split('/')
        .last()
        .unwrap_or("-");
    let default_pipeline_name = String::from("-");
    let pipeline_name = alert.data.pipeline_name.as_ref().unwrap_or(&default_pipeline_name);

    let execution_time = &essentials.fired_date_time;

    // Error message from alertContext -> condition -> allOf -> [0].search_query (or .message)
    let mut error_message = alert.data.message.clone().unwrap_or_else(|| String::from("(no error message found)"));

    // Create Tera context
    let mut context = Context::new();
    context.insert("alert_rule", &essentials.alert_rule);
    context.insert("fired_time", execution_time);
    context.insert("condition", &essentials.monitor_condition);
    context.insert("severity", &essentials.severity);

    // Custom fields (ตาม template)
    context.insert("resource_group", resource_group);
    context.insert("app_name", app_name);
    context.insert("adf_name", adf_name);
    context.insert("pipeline_name", pipeline_name);
    context.insert("execution_time", execution_time);
    context.insert("error_message", &error_message);

    // Render
    let html = tera.render(template_name, &context)?;
    Ok(html)
}