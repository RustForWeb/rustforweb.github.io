use radix_yew_themes::{AccentColor, Theme};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::index::Index;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Index,
}

fn render(route: Route) -> Html {
    match route {
        Route::Index => html! { <Index /> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <Theme accent_color={AccentColor::Orange}>
            <BrowserRouter>
                <Switch<Route> render={render} />
            </BrowserRouter>
        </Theme>
    }
}
