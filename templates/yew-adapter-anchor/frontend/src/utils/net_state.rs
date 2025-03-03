use std::{cell::RefCell, rc::Rc};

use web_sys::Window;
use yew::{platform::spawn_local, Reducible, UseReducerHandle, UseStateHandle};

use crate::{ClusterNetState, FetchReq};

pub type NetState = UseReducerHandle<NetStateInfo>;

#[derive(Debug, PartialEq)]
pub(crate) enum NetStateOptions {
    Ping {
        endpoint: String,
        window: Window,
        trigger: UseStateHandle<bool>,
    },
    Waiting(UseStateHandle<bool>),
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub(crate) struct NetStateInfo {
    pub(crate) state: RefCell<ClusterNetState>,
}

impl Reducible for NetStateInfo {
    type Action = NetStateOptions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let self_inner = self.clone();

        match action {
            NetStateOptions::Ping {
                endpoint,
                window,
                trigger,
            } => {
                *self_inner.state.borrow_mut() = ClusterNetState::Waiting;

                spawn_local(async move {
                    web_sys::console::log_1(
                        &format!("BEFORE PING: {:?}", &self_inner.state.borrow_mut()).into(),
                    );
                    let net_state = FetchReq::ping(&endpoint, &window).await;
                    web_sys::console::log_1(&format!("ENDPOINT: {:?}", &endpoint).into());
                    *self_inner.state.borrow_mut() = net_state;

                    web_sys::console::log_1(
                        &format!("SET STATE: {:?}", &self_inner.state.borrow_mut()).into(),
                    );

                    trigger.set(true);
                });
            }
            NetStateOptions::Waiting(trigger) => {
                *self_inner.state.borrow_mut() = ClusterNetState::Waiting;
                trigger.set(true);
            }
        }

        self
    }
}
