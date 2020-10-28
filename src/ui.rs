#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use iced::{Align, button, Button, Column, Element, Text, Sandbox};
use std::fmt as fmt;
use std::default::Default as Default;

// State information for UI
pub struct WtTable {
    name: String,
    total_entries: u32,
}

// A single line of operation, representing, say "insert 100 foo"
// to insert 100 items into "table:foo".
pub enum WtOperation {
    Insert(u32, WtTable),
    Update(u32, WtTable),
    Delete(u32, WtTable),
    Drop(WtTable)
}

pub struct WtOperationState {
    operation: WtOperation,
    completed: u32,
}

//#[derive(Default)]
pub struct UIState {
    operations: Vec<WtOperation>,
    add_button: button::State,
}

// Message passed for UI
//#[derive(Debug, Clone)]
#[derive(Debug, Clone)]
pub enum Message {
    AddOperation,
    Progress(WtOperation),
}

impl fmt::Debug for WtOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WtOperation::Insert(count, table) => {
                f.debug_struct("Insert")
                    .field("count", &count)
                    .field("table.name", &table.name)
                    .finish()
                //write!(f, "insert {} to {}", count, table.name);
            }
            WtOperation::Update(count, table) => {
                f.debug_struct("Update")
                    .field("count", &count)
                    .field("table.name", &table.name)
                    .finish()
                //write!(f, "update {} in {}", count, table.name);
            }
            WtOperation::Delete(count, table) => {
                f.debug_struct("Delete")
                    .field("count", &count)
                    .field("table.name", &table.name)
                    .finish()
                //write!(f, "delete {} from {}", count, table.name);
            }
            WtOperation::Drop(table) => {
                f.debug_struct("Delete")
                    .field("table.name", &table.name)
                    .finish()
                //write!(f, "drop {}", table.name);
            }
        }
    }
}

impl std::default::Default for UIState {
    //fn default() -> Self { UIState(operations: vec!([]), add_button: <iced::button::State as Trait>::default()) }
    fn default() -> Self {
        let ops_default = vec!([]);
        //return UIState(ops_default, <iced::button::State as Trait>::Released)
        //return UIState(ops_default, Released)
        return (ops_default, iced::button::State::Released);
    }
}

impl Clone for WtOperation {
    fn clone(&self) -> Self {
        match self {
            WtOperation::Insert(count, table) => WtOperation::Insert(*count, *table),
            WtOperation::Update(count, table) => WtOperation::Update(*count, *table),
            WtOperation::Delete(count, table) => WtOperation::Delete(*count, *table),
            WtOperation::Drop(table) => WtOperation::Drop(*table),
        }
    }
}

// View logic for UI

impl Sandbox for UIState {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("WT MT Test")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddOperation => {
                //TODO: self.value += 1;
            }
            Message::Progress(op) => {
                //TODO: self.value -= 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout

        // List of operations that have been issued.
        // TODO: where to put the progress meter?
        let mut operations_text : &str = "first line\nsecond line"; //TODO: self.operations.join("\n");
        Column::new()
            .padding(20)
            .align_items(Align::Start)
            .push(
                // We show the current operations here
                Text::new(&operations_text.to_string()).size(50),
            )
            .push(
                // The decrement button. We tell it to produce a
                // `DecrementPressed` message when pressed
                Button::new(&mut self.add_button, Text::new("Add"))
                    //TODO: .on_press(Message::AddOperation),
            )
            .into()
    }    
}
