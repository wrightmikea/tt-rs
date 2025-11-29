//! HelpPanel component with tutorial accordion sections.
//!
//! Provides a comprehensive help panel for new users.

mod advanced;
mod basics;

use crate::accordion::{Accordion, AccordionSection};
use crate::slide_panel::SlidePanel;
use yew::prelude::*;

/// Properties for the HelpPanel component.
#[derive(Properties, Clone, PartialEq)]
pub struct HelpPanelProps {
    /// Whether the help panel is open.
    pub is_open: bool,
    /// Callback when close is requested.
    pub on_close: Callback<()>,
}

/// Help panel with tutorial sections.
#[function_component(HelpPanel)]
pub fn help_panel(props: &HelpPanelProps) -> Html {
    html! {
        <SlidePanel
            is_open={props.is_open}
            on_close={props.on_close.clone()}
            title="Help & Tutorials"
        >
            <Accordion>
                <AccordionSection title="Getting Started" default_open={true}>
                    { basics::getting_started_content() }
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
                <AccordionSection title="Tips & Tricks">
                    { advanced::tips_content() }
                </AccordionSection>
                <AccordionSection title="About ToonTalk">
                    { advanced::about_content() }
                </AccordionSection>
            </Accordion>
        </SlidePanel>
    }
}
