/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::XRInputSourceBinding;
use crate::dom::bindings::reflector::{reflect_dom_object, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot};
use crate::dom::globalscope::GlobalScope;
use crate::dom::xrsession::XRSession;
use dom_struct::dom_struct;
use webvr_traits::{WebVRGamepadData, WebVRGamepadState};

#[dom_struct]
pub struct XRInputSource {
    reflector: Reflector,
    session: Dom<XRSession>,
    #[ignore_malloc_size_of = "Defined in rust-webvr"]
    data: WebVRGamepadData,
    #[ignore_malloc_size_of = "Defined in rust-webvr"]
    state: DomRefCell<WebVRGamepadState>,
}

impl XRInputSource {
    pub fn new_inherited(
        session: &XRSession,
        data: WebVRGamepadData,
        state: WebVRGamepadState,
    ) -> XRInputSource {
        XRInputSource {
            reflector: Reflector::new(),
            session: Dom::from_ref(session),
            data,
            state: DomRefCell::new(state),
        }
    }

    pub fn new(
        global: &GlobalScope,
        session: &XRSession,
        data: WebVRGamepadData,
        state: WebVRGamepadState,
    ) -> DomRoot<XRInputSource> {
        reflect_dom_object(
            Box::new(XRInputSource::new_inherited(session, data, state)),
            global,
            XRInputSourceBinding::Wrap,
        )
    }
}
