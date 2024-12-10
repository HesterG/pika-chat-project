use yew::prelude::*;
use yew_router::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use crate::services::websocket::WebSocketService;
use crate::routes::Route;
use web_sys::HtmlInputElement;

pub enum Msg {
    SendMessage,
    ReceiveMessage(String),
    UpdateMessageInput(String),
    WebSocketConnected,
    WebSocketDisconnected,
    WebSocketError(String),
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub room_id: i64,
}

pub struct ChatRoom {
    // token: Option<String>,
    ws_service: Option<WebSocketService>,
    message_input: String,
    messages: Vec<String>,
    error: Option<String>,
}

impl Component for ChatRoom {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        // let token = LocalStorage::get::<String>("jwtToken").ok();

        Self {
            // token,
            ws_service: None,
            message_input: String::new(),
            messages: vec![],
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SendMessage => {
                if let Some(ws_service) = &mut self.ws_service {
                    ws_service.send_message(&self.message_input);
                    self.message_input.clear();
                }
                true
            }
            Msg::ReceiveMessage(message) => {
                self.messages.push(message);
                true
            }
            Msg::UpdateMessageInput(input) => {
                self.message_input = input;
                true
            }
            Msg::WebSocketConnected => {
                self.error = None;
                true
            }
            Msg::WebSocketDisconnected => {
                self.error = Some("Disconnected from the server.".to_string());
                true
            }
            Msg::WebSocketError(error) => {
                self.error = Some(error);
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if self.ws_service.is_none() {
            let room_id = ctx.props().room_id;
            let link = ctx.link().clone();

            let link_on_message = link.clone();
            let on_message = Callback::from(move |msg: String| {
                link_on_message.send_message(Msg::ReceiveMessage(msg));
            });

            let link_on_error = link.clone();
            let on_error = Callback::from(move |err: String| {
                link_on_error.send_message(Msg::WebSocketError(err));
            });

            let on_connect = link.callback(|_| Msg::WebSocketConnected);

            let ws_service = WebSocketService::new(
                &room_id.to_string(),
                on_message,
                // token,
                on_error,
                on_connect,
            );

            self.ws_service = Some(ws_service);

        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_message_input = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateMessageInput(input.value())
        });

        let on_send_message = ctx.link().callback(|_| Msg::SendMessage);

        html! {
            <div style="min-height: 100vh; display: flex; flex-direction: column; background-color: #f9fafb;">
                // Header Section
                <header style="background-color: #facc15; padding: 1rem; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); position: static; width: 100%;">
                    <nav style="max-width: 1200px; margin: 0 auto; display: flex; justify-content: space-between; align-items: center;">
                        <div style="font-size: 1.5rem; font-weight: bold; color: #1f2937;">
                            <a href="/">{"Pika Chat"}</a>
                        </div>
                        <div style="display: flex; gap: 1rem;">
                            <a href="/dashboard" style="color: #1f2937; text-decoration: none; font-weight: 500; transition: color 0.2s;">
                                {"Back to Dashboard"}
                            </a>
                        </div>
                    </nav>
                </header>

                // Main Section
                <main style="flex: 1; padding: 2rem; display: flex; flex-direction: column; align-items: center; text-align: center;">
                    <h1 style="font-size: 2.5rem; font-weight: bold; color: #1f2937; margin-bottom: 1.5rem;">
                        { format!("Chat Room: {}", ctx.props().room_id) }
                    </h1>
                    <div style="width: 100%; max-width: 800px; margin-bottom: 2rem; text-align: left;">
                        <div style="border: 1px solid #e5e7eb; border-radius: 0.5rem; padding: 1rem; max-height: 400px; overflow-y: auto; background-color: #ffffff;">
                            {
                                for self.messages.iter().map(|message| html! {
                                    <p style="padding: 0.5rem 0; border-bottom: 1px solid #e5e7eb;">{ message }</p>
                                })
                            }
                        </div>
                        <div style="display: flex; margin-top: 1rem;">
                            <input
                                type="text"
                                value={self.message_input.clone()}
                                oninput={on_message_input}
                                placeholder="Type your message"
                                style="flex: 1; padding: 0.75rem; border: 1px solid #e5e7eb; border-radius: 0.5rem; margin-right: 0.5rem;"
                            />
                            <button
                                onclick={on_send_message}
                                style="padding: 0.75rem 1.5rem; background-color: #1f2937; color: #ffffff; border: none; border-radius: 0.5rem; cursor: pointer; transition: background-color 0.2s;"
                            >
                                {"Send"}
                            </button>
                        </div>
                        if let Some(error) = &self.error {
                            <p style="color: red; margin-top: 1rem;">{ format!("Error: {}", error) }</p>
                        }
                    </div>
                </main>

                // Footer Section
                <footer style="background-color: #1f2937; color: #e5e7eb; text-align: center; padding: 1rem; width: 100%;">
                    <p style="font-size: 0.875rem;">
                        {"© 2024 Pika Chat. All rights reserved."}
                    </p>
                </footer>
            </div>
        }
    }
}
