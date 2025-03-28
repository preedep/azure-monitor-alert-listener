use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureMonitorAlert {
    #[serde(rename = "schemaId")]
    pub schema_id: String,
    pub data: AlertData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertData {
    pub essentials: Essentials,
    #[serde(default)]
    #[serde(rename = "customProperties")]
    pub custom_properties: Option<serde_json::Value>,
    #[serde(default)]
    #[serde(rename = "alertContext")]
    pub alert_context: Option<AlertContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Essentials {
    #[serde(rename = "alertContextVersion")]
    pub alert_context_version: String,
    #[serde(rename = "alertId")]
    pub alert_id: String,
    #[serde(rename = "alertRule")]
    pub alert_rule: String,
    #[serde(rename = "alertRuleID")]
    pub alert_rule_id: String,
    #[serde(rename = "alertTargetIDs")]
    pub alert_target_ids: Vec<String>,
    #[serde(rename = "configurationItems")]
    pub configuration_items: Vec<String>,
    pub description: String,
    #[serde(rename = "essentialsVersion")]
    pub essentials_version: String,
    #[serde(rename = "firedDateTime")]
    pub fired_date_time: String,
    #[serde(rename = "investigationLink")]
    pub investigation_link: String,
    #[serde(rename = "monitorCondition")]
    pub monitor_condition: String,
    #[serde(rename = "monitoringService")]
    pub monitoring_service: String,
    #[serde(rename = "originAlertId")]
    pub origin_alert_id: String,
    pub severity: String,
    #[serde(rename = "signalType")]
    pub signal_type: String,
    #[serde(rename = "targetResourceGroup")]
    pub target_resource_group: String,
    #[serde(rename = "targetResourceType")]
    pub target_resource_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertContext {
    #[serde(rename = "conditionType")]
    pub condition_type: String,
    pub condition: Condition,
    #[serde(default)]
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    #[serde(default)]
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<ConditionDetail>>,

    #[serde(default)]
    #[serde(rename = "staticThresholdFailingPeriods")]
    pub static_threshold_failing_periods: Option<FailingPeriods>,

    #[serde(rename = "windowStartTime")]
    pub window_start_time: String,
    #[serde(rename = "windowEndTime")]
    pub window_end_time: String,
    #[serde(rename = "windowSize")]
    pub window_size: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionDetail {
    #[serde(default)]
    pub dimensions: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub event: Option<serde_json::Value>,
    #[serde(default)]
    #[serde(rename = "metricName")]
    pub metric_name: Option<String>,
    #[serde(default)]
    #[serde(rename = "metricNamespace")]
    pub metric_namespace: Option<String>,
    #[serde(default)]
    #[serde(rename = "metricValue")]
    pub metric_value: Option<f64>,
    #[serde(default)]
    pub operator: Option<String>,
    #[serde(default)]
    pub threshold: Option<String>,
    #[serde(default)]
    #[serde(rename = "timeAggregation")]
    pub time_aggregation: Option<String>,
    #[serde(default)]
    #[serde(rename = "searchQuery")]
    pub search_query: Option<String>,
    #[serde(default)]
    #[serde(rename = "linkToSearchResultsUI")]
    pub link_to_search_results_ui: Option<String>,
    #[serde(default)]
    #[serde(rename = "linkToSearchResultsAPI")]
    pub link_to_search_results_api: Option<String>,
    #[serde(default)]
    #[serde(rename = "linkToFilteredSearchResultsUI")]
    pub link_to_filtered_search_results_ui: Option<String>,
    #[serde(default)]
    #[serde(rename = "linkToFilteredSearchResultsAPI")]
    pub link_to_filtered_search_results_api: Option<String>,
    #[serde(default)]
    #[serde(rename = "metricMeasureColumn")]
    pub metric_measure_column: Option<String>,
    #[serde(default)]
    #[serde(rename = "failingPeriods")]
    pub failing_periods: Option<FailingPeriods>,
    #[serde(default)]
    #[serde(rename = "webTestName")]
    pub web_test_name: Option<String>,
    #[serde(default)]
    #[serde(rename = "targetResourceTypes")]
    pub target_resource_types: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailingPeriods {
    #[serde(rename = "minFailingPeriodsToAlert")]
    pub min_failing_periods_to_alert: u32,
    #[serde(rename = "numberOfEvaluationPeriods")]
    pub number_of_evaluation_periods: u32,
}