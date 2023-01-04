#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct NotificationEventData {
    /// Object containing the names of the attributes that have changed, and their previous values (sent along only with *.updated events).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<NotificationEventDataPreviousAttributes>,
}

/// Object containing the names of the attributes that have changed, and their previous values (sent along only with *.updated events).
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct NotificationEventDataPreviousAttributes {}
