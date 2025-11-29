//! Messaging tutorial content - Birds and Nests (tt2 level).

use yew::prelude::*;

pub fn messaging_content() -> Html {
    html! {
        <div class="help-section">
            <h4>{"What are Birds and Nests?"}</h4>
            <p>
                {"Birds and Nests enable message passing - a bird carries \
                 items to its paired nest. Think of it like carrier pigeons!"}
            </p>

            <h4>{"Current Status"}</h4>
            <p>
                {"Hatching and basic message delivery work. \
                 Robots using nests as input is planned for a future update."}
            </p>

            <h4>{"Hatching: Create a Bird/Nest Pair"}</h4>
            <p>{"Click the Nest icon to create a paired bird and nest:"}</p>
            <ul>
                <li>{"A Nest appears where you clicked"}</li>
                <li>{"A Bird appears next to it, already linked"}</li>
                <li>{"This bird will always deliver to THIS nest"}</li>
            </ul>

            <h4>{"Sending a Message"}</h4>
            <ol>
                <li>{"Create a number (click +1 or +5)"}</li>
                <li>{"Drag the number onto the Bird"}</li>
                <li>{"The number is consumed and appears at the Nest"}</li>
            </ol>

            <h4>{"Try It Now"}</h4>
            <div class="help-example">
                <p>{"1. Click the Nest icon to hatch a bird/nest pair"}</p>
                <p>{"2. Drag the bird away from the nest"}</p>
                <p>{"3. Click +5 to create a number"}</p>
                <p>{"4. Drag the 5 onto the bird"}</p>
                <p>{"5. The 5 appears at the nest!"}</p>
            </div>

            <h4>{"Why Use Messaging?"}</h4>
            <p>
                {"Message passing is fundamental to concurrent programming. \
                 Once robots can watch nests, you'll be able to:"}
            </p>
            <ul>
                <li>{"Send work to robots in different locations"}</li>
                <li>{"Collect results from parallel computations"}</li>
                <li>{"Create pipelines where output flows to the next stage"}</li>
            </ul>
            <p class="help-tip">
                {"Coming soon: Robots that wait for messages, enabling \
                 true concurrent programming patterns."}
            </p>
        </div>
    }
}
