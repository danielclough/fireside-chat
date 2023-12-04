use crate::components::{chat::index::ChatBox, sidebar::index::SideBar};
use leptonic::prelude::*;
use leptos::*;
use leptos_icons::{BsIcon, FaIcon};

#[component]
pub fn App() -> impl IntoView {
    let (drawer_state, set_drawer_state) = create_signal(false);

    view! {
        <Root default_theme=LeptonicTheme::default()>
            <Box id="header">
                // Toggle Sidebar
                <Toggle state=drawer_state set_state=set_drawer_state
                    icons=ToggleIcons {
                    on: FaIcon::FaXmarkSolid.into(),
                    off: BsIcon::BsList.into(),
                }/>
                // Title
                <H1 style="padding: 0; margin: .25rem .5rem;">
                    Candle Chat
                </H1>
                // Toggle Theme
                <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark/>
            </Box>
            <Box id="main-area">
                <ChatBox />
                <Drawer id="sidebar-container" side=DrawerSide::Left shown=drawer_state>
                    <SideBar />
                </Drawer>
            </Box>
        </Root>
    }
}
