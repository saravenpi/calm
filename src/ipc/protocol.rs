use serde::{Deserialize, Serialize};

/// Messages that can be sent between browser components via IPC.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum BrowserMessage {
    #[serde(rename = "switch_tab")]
    SwitchTab {
        #[serde(rename = "tabId")]
        tab_id: usize,
    },

    #[serde(rename = "close_tab")]
    CloseTab {
        #[serde(rename = "tabId")]
        tab_id: usize,
    },

    #[serde(rename = "new_tab")]
    NewTab,

    #[serde(rename = "navigate")]
    Navigate {
        #[serde(rename = "tabId")]
        tab_id: usize,
        url: String,
    },

    #[serde(rename = "navigate_back")]
    NavigateBack,

    #[serde(rename = "navigate_forward")]
    NavigateForward,

    #[serde(rename = "reload")]
    Reload,

    #[serde(rename = "update_title")]
    UpdateTitle { title: String },

    #[serde(rename = "update_url")]
    UpdateUrl { url: String },

    #[serde(rename = "update_navigation_state")]
    UpdateNavigationState {
        #[serde(rename = "canGoBack")]
        can_go_back: bool,
        #[serde(rename = "canGoForward")]
        can_go_forward: bool,
    },

    #[serde(rename = "toggle_downloads")]
    ToggleDownloads,

    #[serde(rename = "toggle_split_view")]
    ToggleSplitView,

    #[serde(rename = "inspect_element")]
    InspectElement,

    #[serde(rename = "close_window")]
    CloseWindow,
}

/// Information about a browser tab for IPC communication.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TabInfo {
    #[serde(rename = "tabId")]
    pub tab_id: usize,
    pub url: String,
    pub title: Option<String>,
}

/// Messages that can be sent from the tab bar component.
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum TabBarMessage {}
