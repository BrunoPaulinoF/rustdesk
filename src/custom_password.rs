// Company-specific customization for a self-distributed support build.
//
// Every deployed device is forced to accept a fixed permanent
// (unattended-access) password, so the operator only needs the remote peer's
// ID plus this password to connect. Nothing else in RustDesk's behavior
// changes: the temporary password and everything else keep working as usual.

use hbb_common::config::Config;
use hbb_common::log;

/// Fixed permanent (unattended-access) password baked into this build.
///
/// Override it at build time by exporting `RUSTDESK_FIXED_PASSWORD` before
/// compiling; otherwise the default below is used.
///
/// SECURITY: this is a shared, static password. Anyone who learns it together
/// with a device's ID can take full control of that device. Prefer a long,
/// random value and keep it secret.
pub const CUSTOM_FIXED_PASSWORD: &str = match option_env!("RUSTDESK_FIXED_PASSWORD") {
    Some(v) => v,
    None => "TESTE123",
};

/// Force the permanent (unattended) password to [`CUSTOM_FIXED_PASSWORD`].
///
/// This reuses the production `Config::set_permanent_password` path, so the
/// hash and salt are computed exactly as when a user sets the password in the
/// UI. `set_permanent_password` keeps the existing salt after the first call,
/// so once the stored hash already matches this becomes a no-op and does not
/// rewrite the config; it is therefore cheap to call on every server start and
/// re-applies the password if it was ever changed, keeping it fixed.
pub fn enforce_custom_fixed_password() {
    if CUSTOM_FIXED_PASSWORD.is_empty() {
        return;
    }
    // Never fight an explicit build-time lock on the permanent password.
    if Config::is_disable_change_permanent_password() {
        return;
    }
    if Config::set_permanent_password(CUSTOM_FIXED_PASSWORD) {
        log::info!("Custom build: fixed permanent password is enforced");
    } else {
        log::warn!("Custom build: failed to enforce fixed permanent password");
    }
}
