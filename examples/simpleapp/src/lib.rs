#![recursion_limit = "512"]
#![allow(non_camel_case_types, unused_attributes)]

// Reference: https://github.com/yewstack/yew textarea example

use yew::{Component, ComponentLink, Html, html, InputData, ShouldRender};

#[cfg(target_arch = "wasm32")]
pub mod rwb;

#[cfg(target_arch = "wasm32")]
fn version_str () -> &'static str {
    return rwb::VERSION_STR;
}

#[cfg(target_arch = "wasm32")]
fn build_timestamp_str () -> &'static str {
    return rwb::BUILD_TIMESTAMP_STR;
}

#[cfg(target_arch = "wasm32")]
fn rwb_version_str () -> &'static str {
    return rwb::RWB_VERSION_STR;
}

#[cfg(not(target_arch = "wasm32"))]
fn version_str () -> &'static str {
    // TODO: Figure out how to do this.  Cargo probably has a way.
    return "X.Y.Z";
}

#[cfg(not(target_arch = "wasm32"))]
fn build_timestamp_str () -> &'static str {
    // TODO: Figure out how to do this.  Cargo probably has a way.
    return "YYYY.MM.DD-hh:mm:ss";
}

#[cfg(not(target_arch = "wasm32"))]
fn rwb_version_str () -> &'static str {
    // TODO: This doesn't apply.  Figure out just what to do.
    return "rwb-vX.X.X";
}

pub struct App {
    link : ComponentLink<Self>,
    value_input : String,
    value_output : String,
}

pub enum Msg {
    GotInput(String),
    Clicked_ClearInput,
    Clicked_CopyToOutput,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create (_ : Self::Properties, link : ComponentLink<Self>) -> Self {
        App {
            link,
            value_input : String::from("hippo"),
            value_output : String::from("ostrich")
        }
    }

    fn update (&mut self, msg : Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.value_input = new_value;
            }
            Msg::Clicked_ClearInput => {
                self.value_input = String::from("");
            }
            Msg::Clicked_CopyToOutput => {
                self.value_output = self.value_input.clone();
            }
        }
        true
    }

    fn change (&mut self, _ : Self::Properties) -> ShouldRender {
        false
    }

    fn view (&self) -> Html {
        html!{
            <div>
                <table style="width:100%">
                    <tr>
                        <th style="width:50%"><div>{ "Input Panel" }</div></th>
                        <th style="width:50%"><div>{ "Output Panel" }</div></th>
                    </tr>
                    <tr>
                        <td><button onclick=self.link.callback(|_| Msg::Clicked_ClearInput)>{ "Clear Input" }</button></td>
                        <td><button onclick=self.link.callback(|_| Msg::Clicked_CopyToOutput)>{ "Copy From Input" }</button></td>
                    </tr>
                    <tr>
                        <td>
                            <textarea
                                style="width:100%"
                                rows=28
                                value=&self.value_input
                                oninput=self.link.callback(|e: InputData| Msg::GotInput(e.value))
                                placeholder="placeholder">
                            </textarea>
                        </td>
                        <td>
                            <textarea
                                style="width:100%"
                                rows=28
                                value=&self.value_output
                                placeholder="placeholder">
                            </textarea>
                        </td>
                    </tr>
                </table>
                <div style="font-size:75%" align="center">{ format!("{} built by ", version_str()) }<a href="https://github.com/vdods/rwb">{ rwb_version_str() }</a>{ format!(" at {}", build_timestamp_str()) }</div>
            </div>
        }
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app () -> Result<(),JsValue> {
    yew::start_app::<App>();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
