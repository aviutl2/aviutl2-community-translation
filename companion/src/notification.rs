use native_dialog::{DialogBuilder, MessageLevel};

pub(super) fn show_update_notification(title: &str, body: &str) -> anyhow::Result<()> {
    DialogBuilder::message()
        .set_level(MessageLevel::Info)
        .set_title(title)
        .set_text(body)
        .alert()
        .show()?;
    Ok(())
}
