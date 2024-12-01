use radix_yew_themes::{
    Box, Card, Flex, FlexAlign, FlexDirection, FlexJustify, FlexWrap, Heading, HeadingAs, Text,
    TextAs,
};
use yew::prelude::*;
use yew_style::Style;

use crate::components::link_button::LinkButton;

#[derive(PartialEq, Properties)]
pub struct ProjectCardProps {
    pub name: String,
    pub description: String,
    pub logo_url: String,
    pub documentation_url: String,
    pub github_url: String,
}

#[function_component]
pub fn ProjectCard(props: &ProjectCardProps) -> Html {
    html! {
        <Card>
            <Flex height="100%" gap=3>
                <Flex align={FlexAlign::Center} min_width="128px">
                    <img
                        src={props.logo_url.clone()}
                        style={Style::from([
                            ("width", "128px"),
                            ("height", "128px"),
                        ])}
                    />
                </Flex>
                <Flex flex_grow="1" direction={FlexDirection::Column} justify={FlexJustify::Between}>
                    <Box flex_grow="1">
                        <Heading r#as={HeadingAs::H2} mb=2>{&props.name}</Heading>
                        <Text r#as={TextAs::P} size=4 mb=2>{&props.description}</Text>
                    </Box>

                    <Flex gap=2 wrap={FlexWrap::Wrap}>
                        <LinkButton size=3 href={props.documentation_url.clone()}>
                            {"Documentation"}
                        </LinkButton>

                        <LinkButton size=3 href={props.github_url.clone()}>
                            {"GitHub"}
                        </LinkButton>
                    </Flex>
                </Flex>
            </Flex>
        </Card>
    }
}
