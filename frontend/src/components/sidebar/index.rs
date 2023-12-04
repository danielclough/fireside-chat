use leptonic::prelude::*;
use leptos::*;

use crate::components::sidebar::{inference::Inference, model::Model};

#[component]
pub fn SideBar() -> impl IntoView {
    view! {
        <Collapsibles default_on_open=OnOpen::CloseOthers>
            <Stack spacing=Size::Em(0.6)>
                <Collapsible open=true>
                    <CollapsibleHeader slot>"Inference Args"</CollapsibleHeader>
                    <CollapsibleBody slot>
                        <Inference />
                    </CollapsibleBody>
                </Collapsible>

                // <Collapsible open=false>
                //     <CollapsibleHeader slot>"Model Args"</CollapsibleHeader>
                //     <CollapsibleBody slot>
                //         <Model />
                //     </CollapsibleBody>
                // </Collapsible>
            </Stack>
        </Collapsibles>
    }
}
