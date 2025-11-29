//! Messaging tutorial content - Birds and Nests (tt2 level).

use yew::prelude::*;

pub fn messaging_content() -> Html {
    html! {
        <div class="help-section">
            <h4>{"What are Birds and Nests?"}</h4>
            <p>
                {"Birds and Nests work together to pass messages. This is how \
                 different parts of your program can communicate!"}
            </p>

            <h4>{"Pairing a Bird with a Nest"}</h4>
            <p>{"Drag a bird onto a nest (or vice versa) to pair them."}</p>
            <ul>
                <li>{"Each bird can only be paired with one nest"}</li>
                <li>{"Once paired, the bird knows where to deliver messages"}</li>
                <li>{"The bird will move near its nest to show the pairing"}</li>
            </ul>

            <h4>{"Sending Messages"}</h4>
            <p>{"To send a message with a paired bird:"}</p>
            <ol>
                <li>{"Drag a number (or other item) onto the bird"}</li>
                <li>{"The bird will \"fly\" to deliver a copy to its nest"}</li>
                <li>{"The message appears near the nest"}</li>
            </ol>

            <h4>{"Why Use Messaging?"}</h4>
            <p>
                {"Messaging allows different robots to communicate. One robot \
                 can send results to another robot's workspace via birds!"}
            </p>
            <p class="help-tip">
                {"Try creating two boxes, each with a robot. Use birds to send \
                 numbers between them!"}
            </p>
        </div>
    }
}
