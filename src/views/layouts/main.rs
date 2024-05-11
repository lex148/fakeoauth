use yew::prelude::*;
use yew::{html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub(crate) fn Layout(props: &LayoutProps) -> Html {
    let turbo = "https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/dist/turbo.es2017-umd.js";

    html! {
      <html lang="en">
        <head>
          <meta charset="UTF-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1.0" />
          <meta http-equiv="X-UA-Compatible" content="ie=edge" />
          <title>{"Dev Fake Login"}</title>
          <link rel="stylesheet" href="/app.css" />
          <link rel="icon" href="./favicon.ico" type="image/x-icon" />
          <script defer=true src={turbo} />
        </head>
        <body class="bg-black/90 text-white">
          <main>
              { props.children.clone() }
          </main>
        </body>
      </html>
    }
}
