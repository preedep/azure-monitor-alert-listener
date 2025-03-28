use tera::{Context, Tera};
use crate::domain::models::AzureMonitorAlert;

pub fn render_alert_email(template_dir: &str,
                          template_name: &str,
                          alert: &AzureMonitorAlert) -> Result<String, Box<dyn std::error::Error>> {
    // Load template
    let tera = Tera::new(template_dir)?;

    // Extract data from alert
    let essentials = &alert.data.essentials;

    // Create Tera context
    let mut context = Context::new();
    context.insert("alert_rule", &essentials.alert_rule);
    context.insert("fired_time", &essentials.fired_date_time);
    context.insert("condition", &essentials.monitor_condition);
    context.insert("severity", &essentials.severity);

    // Render template
    let html = tera.render(template_name, &context)?;
    Ok(html)
}