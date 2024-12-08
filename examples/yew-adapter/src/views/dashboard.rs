use yew::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div class="flex-grow mx-4 lg:mx-auto">
            <div>
                <div class="hero py-[64px]">
                    <div class="hero-content text-center">
                        <div class="max-w-2xl">
                            <h1 class="text-5xl font-bold"> {"gm"} </h1>
                            <p class="py-6"> {"Say hi to your new Solana dApp."} </p>
                        </div>
                    </div>
                </div>
                <div class="max-w-xl mx-auto py-6 sm:px-6 lg:px-8 text-center">
                    <div class="space-y-2">
                        <p> {"Here are some helpful links to get you started."}</p>
                        <div> <a href="https://docs.solana.com/" class="link" target="_blank"
                                rel="noopener noreferrer"> {"Solana Docs"} </a></div>
                        <div> <a href="https://faucet.solana.com/" class="link" target="_blank"
                                rel="noopener noreferrer"> {"Solana Faucet"} </a></div>
                        <div> <a href="https://solanacookbook.com/" class="link" target="_blank"
                                rel="noopener noreferrer"> {"Solana Cookbook"} </a></div>
                        <div> <a href="https://solana.stackexchange.com/" class="link" target="_blank"
                                rel="noopener noreferrer"> {"Solana Stack Overflow"} </a></div>
                        <div> <a href="https://github.com/solana-developers/" class="link" target="_blank"
                                rel="noopener noreferrer"> {"Solana Developers GitHub"} </a></div>
                    </div>
                </div>
            </div>
        </div>
    }
}
