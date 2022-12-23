use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or("light".to_string())]
    pub theme: String, // Todo see how to make this work
    pub children: Children
}

pub struct App {

}

impl Component for App {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        App {}
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("v-application");

        html! {
            <div class={classes}>
                <div class={"v-application__wrap"}>
                    { for ctx.props().children.iter() }
                </div>
            </div>
        }
    }
}
