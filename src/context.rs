use yew::prelude::*;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct LogEntry {
    pub message: String,
    pub level: String, // "info", "success", "warn", "cmd"
}

pub type TerminalContext = UseReducerHandle<TerminalState>;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TerminalState {
    pub logs: Vec<LogEntry>,
}

pub enum TerminalAction {
    Log(String, String), // message, level
    Clear,
}

impl Reducible for TerminalState {
    type Action = TerminalAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TerminalAction::Log(message, level) => {
                let mut logs = self.logs.clone();
                logs.push(LogEntry { message, level });
                Rc::new(Self { logs })
            }
            TerminalAction::Clear => Rc::new(Self { logs: vec![] }),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TerminalProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(TerminalProvider)]
pub fn terminal_provider(props: &TerminalProviderProps) -> Html {
    let msg = use_reducer(TerminalState::default);

    html! {
        <ContextProvider<TerminalContext> context={msg}>
            { props.children.clone() }
        </ContextProvider<TerminalContext>>
    }
}
