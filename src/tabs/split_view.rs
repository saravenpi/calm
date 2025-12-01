use std::collections::HashMap;
use tao::window::Window;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitOrientation {
    Horizontal,
    Vertical,
}

impl SplitOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            SplitOrientation::Horizontal => "horizontal",
            SplitOrientation::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SplitGroup {
    #[allow(dead_code)]
    pub id: usize,
    pub primary_tab_id: usize,
    pub secondary_tab_id: usize,
    pub orientation: SplitOrientation,
    pub split_ratio: f32,
}

impl SplitGroup {
    pub fn new(
        id: usize,
        primary_tab_id: usize,
        secondary_tab_id: usize,
        orientation: SplitOrientation,
    ) -> Self {
        Self {
            id,
            primary_tab_id,
            secondary_tab_id,
            orientation,
            split_ratio: 0.5,
        }
    }

    pub fn toggle_orientation(&mut self) {
        self.orientation = match self.orientation {
            SplitOrientation::Horizontal => SplitOrientation::Vertical,
            SplitOrientation::Vertical => SplitOrientation::Horizontal,
        };
    }

    pub fn swap_panes(&mut self) {
        std::mem::swap(&mut self.primary_tab_id, &mut self.secondary_tab_id);
    }

    pub fn calculate_bounds(
        &self,
        window: &Window,
        sidebar_width: u32,
        download_sidebar_width: Option<u32>,
    ) -> (wry::Rect, wry::Rect) {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor();
        let download_width = download_sidebar_width.unwrap_or(0);

        let sidebar_width_physical = (sidebar_width as f64 * scale_factor) as u32;
        let download_width_physical = (download_width as f64 * scale_factor) as u32;

        let available_width = window_size
            .width
            .saturating_sub(sidebar_width_physical + download_width_physical);
        let available_height = window_size.height;

        let separator_gap = (1.0 * scale_factor) as u32;

        match self.orientation {
            SplitOrientation::Vertical => {
                let split_pos = (available_width as f32 * self.split_ratio) as u32;

                let primary_bounds = wry::Rect {
                    position: tao::dpi::PhysicalPosition::new(sidebar_width_physical as i32, 0)
                        .into(),
                    size: tao::dpi::PhysicalSize::new(
                        split_pos.saturating_sub(separator_gap),
                        available_height,
                    )
                    .into(),
                };

                let secondary_bounds = wry::Rect {
                    position: tao::dpi::PhysicalPosition::new(
                        (sidebar_width_physical + split_pos) as i32,
                        0,
                    )
                    .into(),
                    size: tao::dpi::PhysicalSize::new(
                        available_width - split_pos,
                        available_height,
                    )
                    .into(),
                };

                (primary_bounds, secondary_bounds)
            }
            SplitOrientation::Horizontal => {
                let split_pos = (available_height as f32 * self.split_ratio) as u32;

                let primary_bounds = wry::Rect {
                    position: tao::dpi::PhysicalPosition::new(sidebar_width_physical as i32, 0)
                        .into(),
                    size: tao::dpi::PhysicalSize::new(
                        available_width,
                        split_pos.saturating_sub(separator_gap),
                    )
                    .into(),
                };

                let secondary_bounds = wry::Rect {
                    position: tao::dpi::PhysicalPosition::new(
                        sidebar_width_physical as i32,
                        split_pos as i32,
                    )
                    .into(),
                    size: tao::dpi::PhysicalSize::new(
                        available_width,
                        available_height - split_pos,
                    )
                    .into(),
                };

                (primary_bounds, secondary_bounds)
            }
        }
    }
}

#[derive(Debug)]
pub struct SplitUIState {
    pub active_tab_in_split: bool,
    pub active_group_orientation: Option<String>,
    pub can_create_split: bool,
}

pub struct SplitViewManager {
    groups: HashMap<usize, SplitGroup>,
    next_group_id: usize,
    tab_to_group: HashMap<usize, usize>,
}

impl SplitViewManager {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            next_group_id: 1,
            tab_to_group: HashMap::new(),
        }
    }

    pub fn create_group(
        &mut self,
        primary_tab_id: usize,
        secondary_tab_id: usize,
        orientation: SplitOrientation,
    ) -> usize {
        let group_id = self.next_group_id;
        self.next_group_id += 1;

        let group = SplitGroup::new(group_id, primary_tab_id, secondary_tab_id, orientation);

        self.tab_to_group.insert(primary_tab_id, group_id);
        self.tab_to_group.insert(secondary_tab_id, group_id);
        self.groups.insert(group_id, group);

        group_id
    }

    pub fn remove_group(&mut self, group_id: usize) {
        if let Some(group) = self.groups.remove(&group_id) {
            self.tab_to_group.remove(&group.primary_tab_id);
            self.tab_to_group.remove(&group.secondary_tab_id);
        }
    }

    pub fn get_group_for_tab(&self, tab_id: usize) -> Option<&SplitGroup> {
        self.tab_to_group
            .get(&tab_id)
            .and_then(|group_id| self.groups.get(group_id))
    }

    pub fn get_group_id_for_tab(&self, tab_id: usize) -> Option<usize> {
        self.tab_to_group.get(&tab_id).copied()
    }

    pub fn is_tab_in_group(&self, tab_id: usize) -> bool {
        self.tab_to_group.contains_key(&tab_id)
    }

    pub fn get_non_grouped_tabs(&self, all_tab_ids: &[usize]) -> Vec<usize> {
        all_tab_ids
            .iter()
            .filter(|&&tab_id| !self.is_tab_in_group(tab_id))
            .copied()
            .collect()
    }

    pub fn toggle_group_orientation(&mut self, group_id: usize) {
        if let Some(group) = self.groups.get_mut(&group_id) {
            group.toggle_orientation();
        }
    }

    pub fn swap_group_panes(&mut self, group_id: usize) {
        if let Some(group) = self.groups.get_mut(&group_id) {
            group.swap_panes();
        }
    }

    pub fn remove_tab_from_group(&mut self, tab_id: usize) -> Option<usize> {
        if let Some(group_id) = self.tab_to_group.remove(&tab_id) {
            if let Some(group) = self.groups.get(&group_id) {
                let other_tab_id = if group.primary_tab_id == tab_id {
                    group.secondary_tab_id
                } else {
                    group.primary_tab_id
                };
                self.tab_to_group.remove(&other_tab_id);
            }
            self.groups.remove(&group_id);
            Some(group_id)
        } else {
            None
        }
    }

    pub fn calculate_ui_state(&self, active_tab_id: Option<usize>, all_tab_ids: &[usize]) -> SplitUIState {
        let (active_tab_in_split, active_group_orientation) = if let Some(tab_id) = active_tab_id {
            if let Some(group) = self.get_group_for_tab(tab_id) {
                (true, Some(group.orientation.as_str().to_string()))
            } else {
                (false, None)
            }
        } else {
            (false, None)
        };

        let non_split_tabs = self.get_non_grouped_tabs(all_tab_ids);
        let can_create_split = non_split_tabs.len() >= 2;

        SplitUIState {
            active_tab_in_split,
            active_group_orientation,
            can_create_split,
        }
    }

    pub fn get_split_groups_json(&self) -> String {
        let groups_data: Vec<serde_json::Value> = self.groups.values()
            .map(|group| {
                serde_json::json!({
                    "primary": group.primary_tab_id,
                    "secondary": group.secondary_tab_id,
                    "orientation": group.orientation.as_str()
                })
            })
            .collect();

        serde_json::to_string(&groups_data).unwrap_or_else(|_| "[]".to_string())
    }
}
