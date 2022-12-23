use yew::Classes;

#[derive(Copy, Clone, PartialEq)]
pub enum Size {
    XSmall,
    Small,
    Default,
    Large,
    XLarge
}


// v-icon--size-x-small

impl Size {
    pub fn as_classes(&self, component: &str) -> Classes {
        Classes::from(format!("{}--size-{}", component, self.to_str()))
    }

    fn to_str(&self) -> &str {
        match &self {
            Size::XSmall => "x-small",
            Size::Small => "small",
            Size::Default => "default",
            Size::Large => "large",
            Size::XLarge => "x-large"
        }
    }
}