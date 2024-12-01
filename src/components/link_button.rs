use radix_yew_themes::{
    AccentColorProp, Button, ButtonChildProps, ButtonLoadingProp, ButtonSizeProp,
    ButtonVariantProp, HighContrastProp, Link, MProp, MbProp, MlProp, MrProp, MtProp, MxProp,
    MyProp, RadiusProp,
};
use yew::prelude::*;
use yew_struct_component::Attributes;
use yew_style::Style;

#[derive(PartialEq, Properties)]
pub struct LinkButtonProps {
    #[prop_or_default]
    pub size: ButtonSizeProp,
    #[prop_or_default]
    pub variant: ButtonVariantProp,
    #[prop_or_default]
    pub color: AccentColorProp,
    #[prop_or_default]
    pub high_contrast: HighContrastProp,
    #[prop_or_default]
    pub radius: RadiusProp,
    #[prop_or_default]
    pub loading: ButtonLoadingProp,
    #[prop_or_default]
    pub m: MProp,
    #[prop_or_default]
    pub mx: MxProp,
    #[prop_or_default]
    pub my: MyProp,
    #[prop_or_default]
    pub mt: MtProp,
    #[prop_or_default]
    pub mr: MrProp,
    #[prop_or_default]
    pub mb: MbProp,
    #[prop_or_default]
    pub ml: MlProp,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `a`
    #[prop_or_default]
    pub download: Option<String>,
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub hreflang: Option<String>,
    #[prop_or_default]
    pub ping: Option<String>,
    #[prop_or_default]
    pub referrerpolicy: Option<String>,
    #[prop_or_default]
    pub rel: Option<String>,
    #[prop_or_default]
    pub target: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn LinkButton(props: &LinkButtonProps) -> Html {
    html! {
        <Button
            size={props.size.clone()}
            variant={props.variant.clone()}
            color={props.color.clone()}
            high_contrast={props.high_contrast.clone()}
            radius={props.radius.clone()}
            loading={props.loading.clone()}
            m={props.m.clone()}
            mx={props.mx.clone()}
            my={props.my.clone()}
            mt={props.mt.clone()}
            mr={props.mr.clone()}
            mb={props.mb.clone()}
            ml={props.ml.clone()}

            class={props.class.clone()}
            id={props.id.clone()}
            style={props.style.clone()}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
            as_child={Callback::from({
                let download = props.download.clone();
                let href = props.href.clone();
                let hreflang = props.hreflang.clone();
                let ping = props.ping.clone();
                let referrerpolicy = props.referrerpolicy.clone();
                let rel = props.rel.clone();
                let target = props.target.clone();

                let children = props.children.clone();

                move |ButtonChildProps {
                    node_ref,
                    attributes,

                    class,
                    data_accent_color,
                    data_disabled,
                    data_radius,
                    id,
                    style,

                    ..
                }| html! {
                    <Link
                        class={class}
                        id={id}
                        style={style}

                        download={download.clone()}
                        href={href.clone()}
                        hreflang={hreflang.clone()}
                        ping={ping.clone()}
                        referrerpolicy={referrerpolicy.clone()}
                        rel={rel.clone()}
                        target={target.clone()}

                        node_ref={node_ref}
                        attributes={attributes.with_defaults([
                            ("data-accent-color", Some(data_accent_color)),
                            ("data-disabled", data_disabled),
                            ("data-radius", data_radius),
                        ])}
                    >
                        {children.clone()}
                    </Link>
                }
            })}
        />
    }
}
