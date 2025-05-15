use radix_yew_themes::{
    Breakpoint, Card, Container, Flex, FlexAlign, FlexDirection, FlexWrap, Grid, Heading,
    HeadingAs, Link, ResponsiveValues, Text, TextAs,
};
use yew::prelude::*;
use yew_style::Style;

use crate::components::{link_button::LinkButton, project_card::ProjectCard};

#[function_component]
pub fn Index() -> Html {
    html! {
        <Container p=4>
            <Flex direction={FlexDirection::Column} align={FlexAlign::Center}>
                <img
                    src="/assets/images/logo.svg"
                    style={Style::from([
                        ("width", "256px"),
                        ("height", "256px")
                    ])}
                />
                <Heading
                    size=9
                    style={Style::from([
                        ("color", "var(--accent-a9)")
                    ])}
                    mb=4
                >
                    {"Rust for Web"}
                </Heading>
                <Heading
                    size=7
                    r#as={HeadingAs::H2}
                    mb=4
                >
                        {"Creates and ports web UI libraries for Rust."}
                </Heading>

                <Card>
                    <Text r#as={TextAs::P} size=5 mb=2>
                        {"Rust for Web is a collection of libraries designed for building web applications with Rust and WebAssembly. "}
                        {"These libraries provide ready-to-use components for popular Rust web frameworks such as "}
                        <Link href="https://dioxuslabs.com/">{"Dioxus"}</Link>{", "}
                        <Link href="https://leptos.dev/">{"Leptos"}</Link>{", and "}
                        <Link href="https://yew.rs/">{"Yew"}</Link>{". "}
                        {"Rust for Web is inspired by the rich ecosystems of JavaScript frameworks like React and aims to bring the same ease of use to the Rust web ecosystem.
                        It achieves this by leveraging the strengths of existing JavaScript component libraries and porting their functionality to Rust."}
                    </Text>

                    <Flex gap=2 wrap={FlexWrap::Wrap}>
                        <LinkButton size=3 href="https://github.com/RustForWeb" rel="me">
                            {"GitHub"}
                        </LinkButton>
                        <LinkButton size=3 href="https://discord.gg/nzbN54K2Gr" rel="me">
                            {"Discord"}
                        </LinkButton>
                        <LinkButton size=3 href="https://mastodon.social/@RustForWeb" rel="me">
                            {"Mastodon"}
                        </LinkButton>
                        <LinkButton size=3 href="https://bsky.app/profile/rustforweb.bsky.social" rel="me">
                            {"Bluesky"}
                        </LinkButton>
                        <LinkButton size=3 href="https://x.com/RustForWeb" rel="me">
                            {"X"}
                        </LinkButton>
                    </Flex>
                </Card>
            </Flex>

            <Grid
                mt=4
                columns={ResponsiveValues::from([
                    (Breakpoint::Initial, 1),
                    (Breakpoint::Md, 2),
                ])}
                gap=3
            >
                <ProjectCard
                    name="Rust Floating UI"
                    description="Floating UI is a library that helps you create “floating” elements such as tooltips, popovers, dropdowns, and more."
                    logo_url="/assets/images/rust-floating-ui-logo.svg"
                    documentation_url="https://floating-ui.rustforweb.org"
                    github_url="https://github.com/RustForWeb/floating-ui"
                />

                <ProjectCard
                    name="Rust Lucide"
                    description="Lucide is a beautiful & consistent icon toolkit made by the community."
                    logo_url="/assets/images/rust-lucide-logo.svg"
                    documentation_url="https://lucide.rustforweb.org"
                    github_url="https://github.com/RustForWeb/lucide"
                />

                <ProjectCard
                    name="Rust Radix"
                    description="Radix is a library of components, icons, colors, and templates for building high-quality, accessible UI."
                    logo_url="/assets/images/rust-radix-logo.svg"
                    documentation_url="https://radix.rustforweb.org"
                    github_url="https://github.com/RustForWeb/radix"
                />

                <ProjectCard
                    name="Rust shadcn/ui"
                    description="shadcn/ui is a library of beautifully designed components that you can copy and paste into your apps."
                    logo_url="/assets/images/rust-shadcn-ui-logo.svg"
                    documentation_url="https://shadcn-ui.rustforweb.org"
                    github_url="https://github.com/RustForWeb/shadcn-ui"
                />

                <ProjectCard
                    name="Rust Testing Library"
                    description="Testing Library is a library of simple and complete testing utilities that encourage good testing practices."
                    logo_url="/assets/images/rust-testing-library-logo.svg"
                    documentation_url="https://testing-library.rustforweb.org"
                    github_url="https://github.com/RustForWeb/testing-library"
                />
            </Grid>
        </Container>
    }
}
