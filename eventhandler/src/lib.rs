//MANUAL
//Create an EventHandler
//You can add events and actions with the add_event/add_action functions
//Whenever you want to process the events in the event queue, call process_events

use std::collections::VecDeque;

#[derive(PartialEq)]
pub enum Event {
    // Add event types as needed
}

pub struct Action {
    triggered_by: Event,
    action: Box<dyn FnMut() + Send>,
}
impl Action {
    pub fn new(triggered_by: Event, action: Box<dyn FnMut() + Send>) -> Self {
        Action {
            triggered_by,
            action,
        }
    }
}

pub struct EventHandler {
    event_queue: VecDeque<Event>,
    actions: Vec<Action>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            event_queue: VecDeque::new(),
            actions: vec![],
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn process_events(&mut self) {
        while let Some(event) = self.event_queue.pop_front() {
            for action in &mut self.actions {
                if action.triggered_by == event {
                    (action.action)();
                }
            }
        }
    }
}
