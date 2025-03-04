use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use wallet_adapter::{ConnectionInfo, Wallet, WalletAdapter};
use yew::{platform::spawn_local, Reducible, UseReducerHandle, UseStateHandle};

use super::{AccountInfoState, NotificationInfo};

pub(crate) type GlobalAppState = UseReducerHandle<GlobalAppInfo>;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum GlobalAction {
    Connect {
        wallet: Wallet,
        trigger: UseStateHandle<bool>,
        global_state: GlobalAppState,
    },
    Disconnect {
        trigger: UseStateHandle<bool>,
        account_info_state: AccountInfoState,
    },
    LoadingTrue(UseStateHandle<bool>),
    LoadingFalse(UseStateHandle<bool>),
    ConnectModalTrue(UseStateHandle<bool>),
    ConnectModalFalse(UseStateHandle<bool>),
    DropdownTrue(UseStateHandle<bool>),
    DropdownFalse(UseStateHandle<bool>),
    Message(NotificationInfo),
    RemoveMessage(u32),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct GlobalAppInfo {
    pub(crate) adapter: RefCell<WalletAdapter>,
    pub(crate) active_connection: RefCell<ConnectionInfo>,
    pub(crate) loading: RefCell<bool>,
    pub(crate) connect_modal: RefCell<bool>,
    pub(crate) wallet_dropdown: RefCell<bool>,
    pub(crate) messages: RefCell<VecDeque<NotificationInfo>>,
}

impl GlobalAppInfo {
    pub fn new(adapter: WalletAdapter) -> Self {
        Self {
            adapter: RefCell::new(adapter),
            active_connection: RefCell::default(),
            loading: RefCell::default(),
            connect_modal: RefCell::default(),
            wallet_dropdown: RefCell::default(),
            messages: RefCell::default(),
        }
    }
}

impl Reducible for GlobalAppInfo {
    type Action = GlobalAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            GlobalAction::Connect {
                wallet,
                trigger,
                global_state,
            } => {
                let clone_self = self.clone();

                spawn_local(async move {
                    {
                        *clone_self.active_connection.borrow_mut() = ConnectionInfo::default();
                    }
                    let connect_result = clone_self.adapter.borrow_mut().connect(wallet).await;
                    if let Err(error) = connect_result {
                        global_state
                            .messages
                            .borrow_mut()
                            .push_back(NotificationInfo::error(error));
                    }else {
                        let active = clone_self.adapter.borrow().connection_info().await.clone();

                        *clone_self.active_connection.borrow_mut() = active;

                        *clone_self.loading.borrow_mut() = false;

                        global_state.dispatch(GlobalAction::LoadingFalse(trigger.clone()));
                    }

                    global_state.dispatch(GlobalAction::LoadingFalse(trigger.clone()));

                    trigger.set(true);
                });
            }
            GlobalAction::Disconnect {
                trigger,
                account_info_state,
            } => {
                *account_info_state.balance.borrow_mut() = String::default();
                *account_info_state.token_accounts.borrow_mut() = Vec::default();
                *account_info_state.transactions.borrow_mut() = Vec::default();

                let clone_self = self.clone();
                spawn_local(async move {
                    clone_self.adapter.borrow_mut().disconnect().await;

                    *clone_self.active_connection.borrow_mut() = ConnectionInfo::default();

                    *clone_self.loading.borrow_mut() = false;

                    trigger.set(true);
                });
            }
            GlobalAction::LoadingFalse(trigger) => {
                *self.loading.borrow_mut() = false;
                trigger.set(true);
            }
            GlobalAction::LoadingTrue(trigger) => {
                *self.loading.borrow_mut() = true;
                trigger.set(true);
            }
            GlobalAction::ConnectModalFalse(trigger) => {
                *self.connect_modal.borrow_mut() = false;
                trigger.set(true);
            }
            GlobalAction::ConnectModalTrue(trigger) => {
                *self.connect_modal.borrow_mut() = true;
                trigger.set(true);
            }
            GlobalAction::DropdownFalse(trigger) => {
                *self.wallet_dropdown.borrow_mut() = false;
                trigger.set(true);
            }
            GlobalAction::DropdownTrue(trigger) => {
                *self.wallet_dropdown.borrow_mut() = true;
                trigger.set(true);
            }
            GlobalAction::Message(message) => {
                self.messages.borrow_mut().push_back(message);
            }
            GlobalAction::RemoveMessage(key) => {
                let index = self
                    .messages
                    .borrow()
                    .iter()
                    .enumerate()
                    .find(|(_, notification)| notification.key() == key)
                    .map(|(index, _)| index);

                if let Some(index) = index {
                    self.messages.borrow_mut().remove(index);
                }
            }
        }

        self
    }
}
