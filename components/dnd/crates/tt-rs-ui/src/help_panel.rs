//! HelpPanel component with tutorial accordion sections.
//!
//! Provides a comprehensive help panel for new users.

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
                    { getting_started_content() }
                </AccordionSection>
                <AccordionSection title="Numbers & Arithmetic">
                    { numbers_content() }
                </AccordionSection>
                <AccordionSection title="Boxes & Organization">
                    { boxes_content() }
                </AccordionSection>
                <AccordionSection title="Tools">
                    { tools_content() }
                </AccordionSection>
                <AccordionSection title="Training Robots">
                    { robots_content() }
                </AccordionSection>
                <AccordionSection title="Tips & Tricks">
                    { tips_content() }
                </AccordionSection>
                <AccordionSection title="About ToonTalk">
                    { about_content() }
                </AccordionSection>
            </Accordion>
        </SlidePanel>
    }
}

fn getting_started_content() -> Html {
    html! {
        <div class="help-section">
            <p>{"Welcome to tt-rs, a visual programming environment!"}</p>
            <p>{"Everything is done by dragging and dropping:"}</p>
            <ul>
                <li>{"Click and drag any item to move it"}</li>
                <li>{"Drop items onto each other to combine them"}</li>
                <li>{"Use tools to modify or copy items"}</li>
            </ul>
            <p class="help-tip">
                {"Try dragging the \"+1\" onto the \"0\" to add them together!"}
            </p>
        </div>
    }
}

fn numbers_content() -> Html {
    html! {
        <div class="help-section">
            <h4>{"Number Values"}</h4>
            <p>{"Yellow boxes with numbers are values you can manipulate."}</p>

            <h4>{"Arithmetic Tools"}</h4>
            <p>{"Tools show an operator (+, -, \u{00D7}, \u{00F7}) and a value:"}</p>
            <ul>
                <li><strong>{"+1"}</strong>{" - Adds 1 to a number"}</li>
                <li><strong>{"+5"}</strong>{" - Adds 5 to a number"}</li>
                <li><strong>{"-1"}</strong>{" - Subtracts 1 from a number"}</li>
                <li><strong>{"\u{00D7}2"}</strong>{" - Multiplies by 2"}</li>
                <li><strong>{"\u{00F7}2"}</strong>{" - Divides by 2"}</li>
            </ul>

            <h4>{"Copy Sources"}</h4>
            <p>
                {"Stacked items are \"copy sources\" - click them to create copies \
                 that you can drag around."}
            </p>
        </div>
    }
}

fn boxes_content() -> Html {
    html! {
        <div class="help-section">
            <h4>{"What are Boxes?"}</h4>
            <p>{"Boxes have holes that can hold items. Use them to organize values."}</p>

            <h4>{"Placing Items"}</h4>
            <p>{"Drag any item onto a box hole to place it inside."}</p>

            <h4>{"Splitting Boxes"}</h4>
            <p>
                {"Drag a box onto a number to split it! The number determines \
                 where to split (e.g., drop on \"2\" splits after the 2nd hole)."}
            </p>

            <h4>{"Joining Boxes"}</h4>
            <p>{"Drag one box onto another to join them into a larger box."}</p>

            <h4>{"Creating New Boxes"}</h4>
            <p>
                {"While dragging a box, press a number key (0-9) then drop - \
                 a new box with that many holes will appear!"}
            </p>
        </div>
    }
}

fn tools_content() -> Html {
    html! {
        <div class="help-section">
            <h4>{"Scales"}</h4>
            <p>
                {"Drop numbers on the left or right pan to compare them. \
                 The scales will tip toward the heavier side."}
            </p>

            <h4>{"Vacuum"}</h4>
            <p>
                {"The vacuum erases things! Drag it onto:"
            }</p>
            <ul>
                <li>{"A box hole to erase its contents"}</li>
                <li>{"A free number to delete it"}</li>
            </ul>

            <h4>{"Magic Wand"}</h4>
            <p>
                {"The wand copies things! Drag it onto any item to create \
                 a duplicate."}
            </p>
        </div>
    }
}

fn robots_content() -> Html {
    html! {
        <div class="help-section">
            <h4>{"What are Robots?"}</h4>
            <p>
                {"Robots can learn by watching you! They remember your actions \
                 and can repeat them automatically."}
            </p>

            <h4>{"Training a Robot"}</h4>
            <ol>
                <li>{"Click the robot to start training (it turns yellow)"}</li>
                <li>{"Perform actions - the robot watches and records them"}</li>
                <li>{"Click the robot again to stop training"}</li>
            </ol>

            <h4>{"Running a Trained Robot"}</h4>
            <p>
                {"Once trained, click the robot to run its recorded actions. \
                 It will turn green while working."}
            </p>

            <h4>{"Copying Robots"}</h4>
            <p>
                {"Use the magic wand on a trained robot to create a copy \
                 with the same training!"}
            </p>
        </div>
    }
}

fn tips_content() -> Html {
    html! {
        <div class="help-section">
            <ul>
                <li>
                    <strong>{"Undo mistakes: "}</strong>
                    {"Use the wand to copy before experimenting"}
                </li>
                <li>
                    <strong>{"Precise placement: "}</strong>
                    {"Drop items carefully - they land where you release"}
                </li>
                <li>
                    <strong>{"Tools are persistent: "}</strong>
                    {"The vacuum and wand stay where you drop them"}
                </li>
                <li>
                    <strong>{"Try things! "}</strong>
                    {"The best way to learn is to experiment"}
                </li>
            </ul>
        </div>
    }
}

fn about_content() -> Html {
    html! {
        <div class="help-section">
            <h4>{"History"}</h4>
            <p>
                {"ToonTalk is a visual programming environment originally created by \
                 Ken Kahn. It teaches programming through animated metaphors where \
                 robots learn by watching."}
            </p>

            <h4>{"Previous Versions"}</h4>
            <ul>
                <li>
                    <strong>{"ToonTalk (C++, 1992-2009)"}</strong>
                    {" - The original Windows application"}
                </li>
                <li>
                    <strong>{"ToonTalk Reborn (JavaScript, 2014-2017)"}</strong>
                    {" - Browser-based reimplementation using jQuery"}
                </li>
                <li>
                    <strong>{"tt-rs (Rust/WebAssembly, 2025)"}</strong>
                    {" - This modern reimplementation"}
                </li>
            </ul>

            <h4>{"Learn More"}</h4>
            <p>
                <a href="https://www.toontalk.com/" target="_blank" rel="noopener">
                    {"ToonTalk Website"}
                </a>
                {" - Official site by Ken Kahn"}
            </p>
            <p>
                <a href="https://en.wikipedia.org/wiki/ToonTalk" target="_blank" rel="noopener">
                    {"Wikipedia Article"}
                </a>
                {" - Background and history"}
            </p>
            <p>
                <a href="https://github.com/ToonTalk/ToonTalk" target="_blank" rel="noopener">
                    {"ToonTalk Reborn (GitHub)"}
                </a>
                {" - JavaScript version source code"}
            </p>
        </div>
    }
}
