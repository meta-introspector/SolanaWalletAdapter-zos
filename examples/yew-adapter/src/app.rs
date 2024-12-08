use std::{cell::RefCell, rc::Rc};

use wallet_adapter::WalletAdapter;
use yew::prelude::*;
use yew_router::prelude::*;

// use crate::{SignAndSendTxComponent, SignInComponent, SignMessageComponent, SignTxComponent};
use crate::{Dashboard, Footer, Header};

pub(crate) type YewAdapter = Rc<RefCell<WalletAdapter>>;

#[function_component(App)]
pub fn app() -> Html {
    let init_adapter = WalletAdapter::init().unwrap();

    let adapter = use_state(|| Rc::new(RefCell::new(init_adapter)));

    html! {
        <ContextProvider<YewAdapter> context={(*adapter).clone()}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<YewAdapter>>

    }
}

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <div id="root">
                <div class="h-full flex flex-col">
                    <Header/>
                    <Dashboard/>
                    <Footer/>
                </div>
            </div>
        },
        Route::NotFound => html! { <h1>{ "Page Not Found" }</h1> },
    }
}
