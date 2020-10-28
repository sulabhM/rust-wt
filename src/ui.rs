#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use iced::{Align, button, Button, Column, Element, Text, Sandbox};
use std::fmt as fmt;
use std::default::Default as Default;

////////////////////////////////////////////////////////////////
// State information for UI
#[derive(Clone)]

// A WiredTiger table, and its current state.
// This object must be shared.
pub struct WtTable {
    name: String,
    total_entries: u32,     // number of entries already inserted into the table.
}

// A single line of operation, representing, say "insert 100 foo"
// to insert 100 items into "table:foo".
pub enum WtOperation {
    Insert(u32, WtTable),
    Update(u32, WtTable),
    Delete(u32, WtTable),
    Drop(WtTable)
}

// The state of an operation.
pub struct WtOperationState {
    operation: WtOperation,
    completed: u32,      // number of items completed (inserted/updated/deleted).  Drop only has one item.
}

//#[derive(Default)]
// The entire state of the Operations side of UI
pub struct OperationPaneState {
    operations: Vec<WtOperation>,
    add_button: button::State,
}

////////////////////////////////////////////////////////////////
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

impl std::default::Default for OperationPaneState {
    fn default() -> Self {
        return OperationPaneState{operations: Vec::<WtOperation>::default(), add_button: iced::button::State::default()};
    }
}

impl Clone for WtOperation {
    // TODO: we don't really want to clone the table, it should be shared.
    fn clone(&self) -> Self {
        match self {
            WtOperation::Insert(count, table) => WtOperation::Insert(*count, table.clone()),
            WtOperation::Update(count, table) => WtOperation::Update(*count, table.clone()),
            WtOperation::Delete(count, table) => WtOperation::Delete(*count, table.clone()),
            WtOperation::Drop(table) => WtOperation::Drop(table.clone()),
        }
    }
}

////////////////////////////////////////////////////////////////
// View and updating logic for UI

impl Sandbox for OperationPaneState {
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
                //TODO: put the operation into the operation list, and start performing the operation
                // in another thread.
            }
            // called to update progress
            Message::Progress(_op) => {
                //TODO: update the progress
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout

        // List of operations that have been issued.
        // TODO: where to put the progress meter?
        let operations_text : &str = "first line\nsecond line"; //TODO: self.operations.join("\n");
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
