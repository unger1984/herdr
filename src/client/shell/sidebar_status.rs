use bytes::Bytes;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};

pub(super) struct SidebarStatusRuntime {
    pub(super) pane_id: crate::layout::PaneId,
    pub(super) command: Vec<String>,
    pub(super) runtime: crate::terminal::TerminalRuntime,
    pub(super) exited: bool,
}

pub(super) fn block_height(height: u16, area_height: u16, active: bool) -> u16 {
    if active {
        height.min(area_height.saturating_sub(4))
    } else {
        0
    }
}

pub(super) struct ExpandedSidebarLayout {
    pub(super) workspace: Rect,
    pub(super) detail: Rect,
    pub(super) section_divider: Rect,
    pub(super) status: Rect,
    pub(super) status_divider: Rect,
}

pub(super) fn expanded_sidebar_layout(
    area: Rect,
    split: f32,
    status_height: u16,
    status_active: bool,
) -> ExpandedSidebarLayout {
    let content = Rect::new(area.x, area.y, area.width.saturating_sub(1), area.height);
    let status_height = block_height(status_height, content.height, status_active);
    let sections = Rect::new(
        area.x,
        area.y,
        area.width,
        area.height.saturating_sub(status_height),
    );
    let (workspace, detail) = crate::ui::expanded_sidebar_sections(sections, split);
    let status_divider = if status_height > 0 {
        Rect::new(
            content.x,
            content.bottom().saturating_sub(status_height),
            content.width,
            1,
        )
    } else {
        Rect::default()
    };
    let status = Rect::new(
        content.x,
        status_divider.y.saturating_add(1),
        content.width,
        status_height.saturating_sub(1),
    );
    ExpandedSidebarLayout {
        workspace,
        detail,
        section_divider: crate::ui::sidebar_section_divider_rect(sections, split),
        status,
        status_divider,
    }
}

pub(super) fn render_divider(buffer: &mut Buffer, divider: Rect, color: Color) {
    if divider.is_empty() {
        return;
    }
    for x in divider.x..divider.right() {
        buffer[(x, divider.y)]
            .set_symbol("─")
            .set_style(Style::default().fg(color));
    }
}

pub(super) fn send_key(
    runtime: Option<&SidebarStatusRuntime>,
    key: crate::input::TerminalKey,
) -> bool {
    runtime.is_some_and(|status| {
        let bytes = status.runtime.encode_terminal_key(key);
        !bytes.is_empty() && status.runtime.try_send_bytes(Bytes::from(bytes)).is_ok()
    })
}

#[cfg(test)]
mod tests {
    use super::block_height;

    #[test]
    fn block_reserves_rows_only_for_a_live_runtime() {
        assert_eq!(block_height(8, 30, false), 0);
        assert_eq!(block_height(8, 30, true), 8);
        assert_eq!(block_height(20, 12, true), 8);
    }
}
