use crate::mdi_icon::MdiIcon;
use yew::prelude::*;
use ycomposables::size_props;

pub struct Icon {

}

#[derive(Properties, Clone, PartialEq)]
#[size_props]
pub struct Props {
    pub icon: MdiIcon,
    #[prop_or_default]
    pub large: bool,
}


impl Component for Icon {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        //ctx.props().
        html! {}
    }
}