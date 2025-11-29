//! Messaging tutorial content - Birds and Nests (tt2 level).

use yew::prelude::*;

pub fn messaging_content() -> Html {
    html! {
        <div class="help-section">
            <h4>{"What are Birds and Nests?"}</h4>
            <p>
                {"Birds and Nests work together to pass messages between \
                 different parts of your program - like sending a letter!"}
            </p>

            <h4>{"Hatching: Creating a Bird/Nest Pair"}</h4>
            <p>{"Click on the Nest in the tool area to \"hatch\" it:"}</p>
            <ul>
                <li>{"A new Nest appears where you clicked"}</li>
                <li>{"A Bird appears next to it, already paired"}</li>
                <li>{"This bird will always deliver to this nest"}</li>
            </ul>

            <h4>{"Sending a Message"}</h4>
            <p>{"To send something via the bird:"}</p>
            <ol>
                <li>{"Create a number (click a number stack)"}</li>
                <li>{"Drag the number onto the Bird"}</li>
                <li>{"The bird delivers it - number appears at the Nest!"}</li>
            </ol>

            <h4>{"Try It: Send the Number 42"}</h4>
            <div class="help-example">
                <p>{"1. Click the Nest stack to hatch a bird/nest pair"}</p>
                <p>{"2. Drag the bird to the right side of the screen"}</p>
                <p>{"3. Click +5 eight times, then +1 twice to make 42"}</p>
                <p>{"4. Drag the 42 onto the bird"}</p>
                <p>{"5. Watch: 42 appears at the nest!"}</p>
            </div>

            <h4>{"Why Use Messaging?"}</h4>
            <p>
                {"Messaging lets robots work together. One robot can compute \
                 a result and send it to another robot via a bird. This is \
                 how real programs coordinate multiple tasks!"}
            </p>
            <p class="help-tip">
                {"Advanced: Create two bird/nest pairs for two-way \
                 communication between robots."}
            </p>
        </div>
    }
}
