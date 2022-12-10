use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // html! {
    //     <h1>{ "Hello Yew!" }</h1>
    // }

    html! {
        <>
            <h1 style="text-align:center;">{ "Joe Chen's 100-day project" }</h1>
            <div>
                <h2 style="text-align:center;">{"Introduction"}</h2>
                <p style="text-align:center;">{ "我的 100 天 Rust 小项目练习挑战"}</p>
                <p style="text-align:center;">{ "Start: 2022-12-10" }</p>
                <p style="text-align:center;">{ "End: 2023-03-19" }</p>
                <p style="text-align:center;">
                    <a href="https://github.com/geometryolife/100">{ "GitHub: geometryolife/100" }</a>
                </p>
            </div>
            <div>
                <h3 style="text-align:center;">{ "The first day" }</h3>
                <p style="text-align:center;">{ "This is a simple Yew web application" }</p>
                <h3 style="text-align:center;">{ "在 Yew 中使用 html! 宏转换 HTML 时有三点需要注意：" }</h3>

                <p style="text-align:center;">{ "Expressions must be wrapped in curly braces ({ })." }</p>
                <p style="text-align:center;">{ "There must only be one root node. If you want to have multiple elements without wrapping them in a container, an empty tag/fragment (<> ... </>) is used" }</p>
                <p style="text-align:center;">{ "Elements must be closed properly." }</p>
                <div style="text-align:center;">
                    <img src="https://yew.rs/img/logo.png" alt="Yew logo" />
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
