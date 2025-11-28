use tao::window::Window;

/// Orientation of the split view divider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitOrientation {
    Horizontal,
    Vertical,
}

/// Identifier for a pane in split view mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitPane {
    Primary,
    Secondary,
}

/// State tracking for split view mode including panes, orientation, and active selection.
#[derive(Debug)]
pub struct SplitViewState {
    pub enabled: bool,
    pub orientation: SplitOrientation,
    pub primary_tab_id: Option<usize>,
    pub secondary_tab_id: Option<usize>,
    pub split_ratio: f32,
    pub active_pane: SplitPane,
}

impl Default for SplitViewState {
    fn default() -> Self {
        Self {
            enabled: false,
            orientation: SplitOrientation::Vertical,
            primary_tab_id: None,
            secondary_tab_id: None,
            split_ratio: 0.5,
            active_pane: SplitPane::Primary,
        }
    }
}

impl SplitViewState {
    /// Creates a new split view state in disabled mode.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables split view mode with the given tabs and orientation.
    pub fn enable(&mut self, primary_tab: usize, secondary_tab: usize, orientation: SplitOrientation) {
        self.enabled = true;
        self.primary_tab_id = Some(primary_tab);
        self.secondary_tab_id = Some(secondary_tab);
        self.orientation = orientation;
        self.active_pane = SplitPane::Primary;
    }

    /// Disables split view mode and clears tab assignments.
    pub fn disable(&mut self) {
        self.enabled = false;
        self.primary_tab_id = None;
        self.secondary_tab_id = None;
    }

    /// Toggles the orientation between horizontal and vertical.
    pub fn toggle_orientation(&mut self) {
        self.orientation = match self.orientation {
            SplitOrientation::Horizontal => SplitOrientation::Vertical,
            SplitOrientation::Vertical => SplitOrientation::Horizontal,
        };
    }

    /// Swaps the primary and secondary tab assignments.
    pub fn swap_panes(&mut self) {
        std::mem::swap(&mut self.primary_tab_id, &mut self.secondary_tab_id);
    }

    /// Sets which pane is currently active.
    pub fn set_active_pane(&mut self, pane: SplitPane) {
        self.active_pane = pane;
    }

    /// Returns the tab ID of the currently active pane.
    pub fn get_active_tab_id(&self) -> Option<usize> {
        match self.active_pane {
            SplitPane::Primary => self.primary_tab_id,
            SplitPane::Secondary => self.secondary_tab_id,
        }
    }

    /// Returns whether a tab is visible in split view mode.
    pub fn is_tab_visible(&self, tab_id: usize) -> bool {
        if !self.enabled {
            return false;
        }
        self.primary_tab_id == Some(tab_id) || self.secondary_tab_id == Some(tab_id)
    }

    /// Calculates the bounds for both panes based on window size and orientation.
    pub fn calculate_bounds(
        &self,
        window: &Window,
        sidebar_width: u32,
        download_sidebar_width: Option<u32>,
    ) -> (wry::Rect, wry::Rect) {
        let window_size = window.inner_size();
        let download_width = download_sidebar_width.unwrap_or(0);

        let available_width = window_size.width.saturating_sub(sidebar_width + download_width);
        let available_height = window_size.height;

        match self.orientation {
            SplitOrientation::Vertical => {
                let split_pos = (available_width as f32 * self.split_ratio) as u32;

                let primary_bounds = wry::Rect {
                    position: tao::dpi::LogicalPosition::new(sidebar_width as i32, 0).into(),
                    size: tao::dpi::LogicalSize::new(split_pos, available_height).into(),
                };

                let secondary_bounds = wry::Rect {
                    position: tao::dpi::LogicalPosition::new(
                        (sidebar_width + split_pos) as i32,
                        0,
                    )
                    .into(),
                    size: tao::dpi::LogicalSize::new(
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
                    position: tao::dpi::LogicalPosition::new(sidebar_width as i32, 0).into(),
                    size: tao::dpi::LogicalSize::new(available_width, split_pos).into(),
                };

                let secondary_bounds = wry::Rect {
                    position: tao::dpi::LogicalPosition::new(sidebar_width as i32, split_pos as i32)
                        .into(),
                    size: tao::dpi::LogicalSize::new(available_width, available_height - split_pos)
                        .into(),
                };

                (primary_bounds, secondary_bounds)
            }
        }
    }

    /// Adjusts the split ratio between panes, clamped between 0.2 and 0.8.
    pub fn adjust_split_ratio(&mut self, delta: f32) {
        self.split_ratio = (self.split_ratio + delta).clamp(0.2, 0.8);
    }
}

/// Manager for split view functionality.
pub struct SplitViewManager {
    state: SplitViewState,
}

impl SplitViewManager {
    /// Creates a new split view manager.
    pub fn new() -> Self {
        Self {
            state: SplitViewState::new(),
        }
    }

    /// Returns an immutable reference to the split view state.
    pub fn state(&self) -> &SplitViewState {
        &self.state
    }

    /// Returns a mutable reference to the split view state.
    pub fn state_mut(&mut self) -> &mut SplitViewState {
        &mut self.state
    }

    /// Toggles split view on/off and returns whether it was enabled.
    pub fn toggle_split_view(
        &mut self,
        current_tab_id: Option<usize>,
        available_tabs: &[usize],
    ) -> bool {
        if self.state.enabled {
            self.state.disable();
            false
        } else {
            if let Some(primary) = current_tab_id {
                let secondary = available_tabs
                    .iter()
                    .find(|&&id| id != primary)
                    .copied();

                if let Some(secondary) = secondary {
                    self.state.enable(primary, secondary, SplitOrientation::Vertical);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
    }

    /// Closes a pane in split view mode and adjusts the layout.
    pub fn close_pane(&mut self, pane: SplitPane) {
        match pane {
            SplitPane::Primary => {
                if let Some(secondary) = self.state.secondary_tab_id {
                    self.state.primary_tab_id = Some(secondary);
                    self.state.disable();
                }
            }
            SplitPane::Secondary => {
                self.state.disable();
            }
        }
    }

    /// Replaces the tab displayed in a specific pane.
    pub fn replace_tab_in_pane(&mut self, pane: SplitPane, new_tab_id: usize) {
        match pane {
            SplitPane::Primary => self.state.primary_tab_id = Some(new_tab_id),
            SplitPane::Secondary => self.state.secondary_tab_id = Some(new_tab_id),
        }
    }
}
