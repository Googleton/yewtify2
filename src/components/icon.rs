use crate::mdi_icon::MdiIcon;
use yew::prelude::*;
use crate::composables::size::Size;

pub struct Icon {

}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub icon: MdiIcon,
    #[prop_or(Size::Default)]
    pub size: Size
}


impl Component for Icon {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Icon { }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("v-icon");

        classes.extend(ctx.props().size.as_classes("v-icon"));
        classes.push("mdi");
        classes.push("mdi-account");

        html! {
            <i class={classes} />
        }
    }
}