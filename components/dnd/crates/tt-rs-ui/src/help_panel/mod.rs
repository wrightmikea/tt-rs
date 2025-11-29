//! HelpPanel component with tutorial accordion sections.
//!
//! Provides a comprehensive help panel for new users.
//! Content is contextual based on the current user level.

mod advanced;
mod basics;
mod messaging;

use crate::accordion::{Accordion, AccordionSection};
use crate::slide_panel::SlidePanel;
use crate::user_level::UserLevel;
use yew::prelude::*;

/// Properties for the HelpPanel component.
#[derive(Properties, Clone, PartialEq)]
pub struct HelpPanelProps {
    /// Whether the help panel is open.
    pub is_open: bool,
    /// Callback when close is requested.
    pub on_close: Callback<()>,
    /// Current user level for contextual help.
    #[prop_or_default]
    pub level: UserLevel,
}

/// Help panel with tutorial sections.
#[function_component(HelpPanel)]
pub fn help_panel(props: &HelpPanelProps) -> Html {
    let title = format!("Help & Tutorials ({})", props.level.name());

    html! {
        <SlidePanel
            is_open={props.is_open}
            on_close={props.on_close.clone()}
            title={title}
        >
            <Accordion>
                <AccordionSection title="Getting Started" default_open={true}>
                    { basics::getting_started_content(props.level) }
                </AccordionSection>
                <AccordionSection title="Numbers & Arithmetic">
                    { basics::numbers_content() }
                </AccordionSection>
                <AccordionSection title="Boxes & Organization">
                    { basics::boxes_content() }
                </AccordionSection>
                <AccordionSection title="Tools">
                    { advanced::tools_content() }
                </AccordionSection>
                <AccordionSection title="Training Robots">
                    { advanced::robots_content() }
                </AccordionSection>
                { render_level_sections(props.level) }
                <AccordionSection title="Tips & Tricks">
                    { advanced::tips_content(props.level) }
                </AccordionSection>
                <AccordionSection title="About ToonTalk">
                    { advanced::about_content() }
                </AccordionSection>
            </Accordion>
        </SlidePanel>
    }
}

/// Render level-specific sections.
fn render_level_sections(level: UserLevel) -> Html {
    match level {
        UserLevel::Tt1 => html! {},
        UserLevel::Tt2 => html! {
            <AccordionSection title="Birds & Nests (Messaging)">
                { messaging::messaging_content() }
            </AccordionSection>
        },
    }
}
