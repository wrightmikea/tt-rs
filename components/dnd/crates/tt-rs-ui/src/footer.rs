//! Footer component with Copyright, License Link, Repository Link, Build info.

use yew::prelude::*;

// Build info from build.rs
const BUILD_COMMIT: &str = env!("BUILD_COMMIT");
const BUILD_HOST: &str = env!("BUILD_HOST");
const BUILD_TIME: &str = env!("BUILD_TIME");

// Copyright info
const COPYRIGHT: &str = "Copyright (c) 2025 Michael A Wright";

// License Link
const LICENSE_URL: &str = "https://github.com/wrightmikea/tt-rs/blob/main/LICENSE";

// Repository Link
const REPO_URL: &str = "https://github.com/wrightmikea/tt-rs";

/// Footer component displaying build and project info.
#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="app-footer">
            <span>{COPYRIGHT}</span>
            <span>{" | "}</span>
            <a href={LICENSE_URL}>{"BSD License"}</a>
            <span>{" | "}</span>
            <a href={REPO_URL}>{"Repository"}</a>
            <span>{" | Build Time: "}</span>
            <span>{BUILD_TIME}</span>
            <span>{" Build Host: "}</span>
            <span>{BUILD_HOST}</span>
            <span>{" Build Commit: "}</span>
            <span>{BUILD_COMMIT}</span>
        </footer>
    }
}
