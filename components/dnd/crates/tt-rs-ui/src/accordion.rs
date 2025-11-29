//! Accordion component for collapsible sections.
//!
//! Used in the help panel to organize tutorial content.

use yew::prelude::*;

/// Properties for an accordion section.
#[derive(Properties, Clone, PartialEq)]
pub struct AccordionSectionProps {
    /// Title shown in the header.
    pub title: AttrValue,
    /// Content shown when expanded.
    pub children: Children,
    /// Whether this section starts expanded.
    #[prop_or(false)]
    pub default_open: bool,
}

/// A single collapsible accordion section.
#[function_component(AccordionSection)]
pub fn accordion_section(props: &AccordionSectionProps) -> Html {
    let expanded = use_state(|| props.default_open);

    let on_toggle = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!*expanded))
    };

    let section_class = format!(
        "accordion-section{}",
        if *expanded { " expanded" } else { "" }
    );

    let icon = if *expanded { "\u{25BC}" } else { "\u{25B6}" };

    html! {
        <div class={section_class}>
            <button class="accordion-header" onclick={on_toggle}>
                <span class="accordion-icon">{ icon }</span>
                <span class="accordion-title">{ &props.title }</span>
            </button>
            if *expanded {
                <div class="accordion-content">
                    { for props.children.iter() }
                </div>
            }
        </div>
    }
}

/// Properties for the Accordion container.
#[derive(Properties, Clone, PartialEq)]
pub struct AccordionProps {
    /// AccordionSection children.
    pub children: Children,
}

/// Container for multiple accordion sections.
#[function_component(Accordion)]
pub fn accordion(props: &AccordionProps) -> Html {
    html! {
        <div class="accordion">
            { for props.children.iter() }
        </div>
    }
}
