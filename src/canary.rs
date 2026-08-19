/// Stable content marker consumed by Gothab's external synthetic monitor.
pub const GOTHAB_STATUS_CANARY_8A6E: &str = "GOTHAB_STATUS_CANARY_8A6E";

/// Stable function symbol consumed by Gothab's external symbol-search probe.
pub fn gothab_status_canary_8a6e() -> &'static str {
    GOTHAB_STATUS_CANARY_8A6E
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canary_marker_is_stable() {
        assert_eq!(gothab_status_canary_8a6e(), "GOTHAB_STATUS_CANARY_8A6E");
    }
}
