//! Adversarial input and edge-case tests for `guise-profiles`.
//!
//! Tests malformed User-Agents, out-of-bounds TTL inputs, invalid TCP option
//! mnemonics, huge input strings, and header name normalizations to ensure
//! fail-closed and panic-free behavior across all public interfaces.

use guise_profiles::{
    canonical_navigation_header_name, infer_initial_ttl, named_profile, os_network_coherence,
    os_network_options_match, profile_os_network_stack, user_agent_facts, ALL_PROFILES,
    NetworkOsCoherence, UserAgentBrowser, UserAgentPlatform,
};

#[test]
fn user_agent_facts_handles_adversarial_and_malformed_strings() {
    // 100KB repetitive string
    let huge_ua = "A".repeat(100_000);
    let facts = user_agent_facts(&huge_ua);
    assert_eq!(facts.browser, UserAgentBrowser::Unknown);
    assert_eq!(facts.platform, UserAgentPlatform::Unknown);
    assert_eq!(facts.browser_major_version, None);
    assert_eq!(facts.inferred_profile, None);

    // Overflow major version number
    let overflow_ua = "Mozilla/5.0 (Windows NT 10.0) Chrome/99999999999999999999999999999.0.0.0 Safari/537.36";
    let facts = user_agent_facts(overflow_ua);
    assert_eq!(facts.browser, UserAgentBrowser::Chrome);
    assert_eq!(facts.platform, UserAgentPlatform::Windows);
    assert_eq!(facts.browser_major_version, None); // Overflow safely returns None

    // Null bytes and control characters
    let null_ua = "Mozilla/5.0 \0 (Windows \t NT \n 10.0) \r Chrome/131.0";
    let facts = user_agent_facts(null_ua);
    assert_eq!(facts.browser, UserAgentBrowser::Chrome);

    // Empty and whitespace-only strings
    assert_eq!(user_agent_facts("").browser, UserAgentBrowser::Unknown);
    assert_eq!(user_agent_facts("   \t\n\r  ").browser, UserAgentBrowser::Unknown);
}

#[test]
fn user_agent_facts_handles_conflicting_and_stacked_tokens() {
    // Conflicting UA containing multiple browser tokens
    let conflicting = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Firefox/150.0 Trident/7.0";
    let facts = user_agent_facts(conflicting);
    // Trident/ takes precedence in UA parsing for legacy IE detection
    assert_eq!(facts.browser, UserAgentBrowser::InternetExplorer);
    assert_eq!(facts.platform, UserAgentPlatform::Windows);
}

#[test]
fn named_profile_handles_adversarial_lookup_strings() {
    assert_eq!(named_profile(""), None);
    assert_eq!(named_profile("   "), None);
    assert_eq!(named_profile("\0chrome\0"), None);
    assert_eq!(named_profile("non_existent_profile_name_12345"), None);

    // Unicode whitespace and case variations
    assert_eq!(
        named_profile("  CHROME-WINDOWS  "),
        named_profile("chrome-windows")
    );
    assert_eq!(named_profile("FIREFOX-LINUX\n"), named_profile("firefox-linux"));
}

#[test]
fn infer_initial_ttl_handles_all_u8_boundary_cases() {
    assert_eq!(infer_initial_ttl(0), 0);
    assert_eq!(infer_initial_ttl(1), 64);
    assert_eq!(infer_initial_ttl(63), 64);
    assert_eq!(infer_initial_ttl(64), 64);
    assert_eq!(infer_initial_ttl(65), 128);
    assert_eq!(infer_initial_ttl(127), 128);
    assert_eq!(infer_initial_ttl(128), 128);
    assert_eq!(infer_initial_ttl(129), 255);
    assert_eq!(infer_initial_ttl(254), 255);
    assert_eq!(infer_initial_ttl(255), 255);
}

#[test]
fn os_network_coherence_is_total_and_safe_across_all_ttls() {
    for profile in ALL_PROFILES {
        for ttl in 0..=255 {
            let coherence = os_network_coherence(*profile, ttl);
            if ttl == 0 {
                assert_eq!(coherence, NetworkOsCoherence::Unknown);
            }
        }
    }
}

#[test]
fn ja4t_fails_closed_on_invalid_or_adversarial_option_mnemonics() {
    let mut stack = profile_os_network_stack(ALL_PROFILES[0]);

    // Malformed option layout
    stack.tcp_options_layout = "mss,invalid_option_kind,ws";
    let result = stack.ja4t();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.unknown_option, "invalid_option_kind");

    // Empty option layout
    stack.tcp_options_layout = "";
    let err = stack.ja4t().unwrap_err();
    assert_eq!(err.unknown_option, "");

    // Options layout matching predicate handles malformed layouts safely
    for profile in ALL_PROFILES {
        assert!(!os_network_options_match(*profile, "invalid_layout_string"));
        assert!(!os_network_options_match(*profile, ""));
    }
}

#[test]
fn canonical_navigation_header_name_normalizes_casing() {
    assert_eq!(canonical_navigation_header_name("user-agent"), "User-Agent");
    assert_eq!(canonical_navigation_header_name("accept"), "Accept");
    assert_eq!(canonical_navigation_header_name("accept-language"), "Accept-Language");
    assert_eq!(canonical_navigation_header_name("accept-encoding"), "Accept-Encoding");
    assert_eq!(
        canonical_navigation_header_name("upgrade-insecure-requests"),
        "Upgrade-Insecure-Requests"
    );
    assert_eq!(canonical_navigation_header_name("sec-fetch-dest"), "Sec-Fetch-Dest");
    assert_eq!(canonical_navigation_header_name("sec-fetch-mode"), "Sec-Fetch-Mode");
    assert_eq!(canonical_navigation_header_name("sec-fetch-site"), "Sec-Fetch-Site");
    assert_eq!(canonical_navigation_header_name("sec-fetch-user"), "Sec-Fetch-User");

    // Passthrough for unknown header names
    assert_eq!(canonical_navigation_header_name("x-custom-header"), "x-custom-header");
}
