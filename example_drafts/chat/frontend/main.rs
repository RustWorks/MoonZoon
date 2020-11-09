use zoon::*;
use shared::{UpMsg, DownMsg, Message};
use std::mem;

zoons!{

    #[model]
    fn username() -> String {
        "John".to_owned()
    }

    #[update]
    fn set_username(username: String) {
        username().set(username);
    }

    #[model]
    fn messages() -> Vec<Model<Message>> {
        Vec::new()
    }

    #[model]
    fn new_message_text() -> String {
        String::new()
    }

    #[update]
    fn set_new_message_text(text: String) {
        new_message_text().set(text);
    }

    #[model]
    fn connection() -> Connection<UpMsg, DownMsg> {
        Connection::new("localhost:9000", |msg| {
            if let DownMsg::MessageReceived(message) = msg {
                messages().update(|messages| messages.push(Model::new(message)));
            }
        })
    }

    #[update]
    fn send_message() {
        connection().use_ref(|connection| {
            connection.send_msg(UpMsg::SendMessage(Message {
                username: username().inner(),
                text: new_message_text().map_mut(mem::take),
            }));
        });
    }

    #[view]
    fn view() -> Column {
        column![
            received_messages(),
            new_message_panel(),
            username_panel(),
        ]
    }

    #[view]
    fn received_messages() -> Column [
        column![
            messages().map(|messages| messages.iter().map(received_message)),
        ]
    ]

    #[view]
    fn received_message(message: Model<Message>) -> Row {
        let message = message.inner();
        row![
            column![
                el![
                    font::bold(),
                    message.username,
                ],
                message.text
            ]
        ]
    }

    #[view]
    fn new_message_panel() -> Row {
        let new_message_text = new_message_text().inner();
        row![
            text_input![
                do_once(focus),
                text_input::on_change(set_new_message_text),
                input::label_hidden("New message text"),
                placeholder![
                    placeholder::text("Message"),
                ],
                on_key_down(|event| {
                    if let Key::Enter = event.key {
                        send_message()
                    }
                }),
                new_message_text,
            ],
            button![
                button::on_press(send_message), 
                "Send",
            ],
        ]
    }

    #[view]
    fn username_panel() -> Row {
        let input_id = use_state(ElementId::new);
        let username = username().inner();
        row![
            label![
                label::for_input(input_id.inner()),
                "Username:",
            ],
            text_input![
                id(input_id.inner()),
                text_input::on_change(set_username),
                placeholder![
                    placeholder::text("Joe"),
                ],
                username,
            ],
        ]
    }

}

fn main() {
    start!(zoons)
}