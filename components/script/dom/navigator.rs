/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::codegen::Bindings::NavigatorBinding;
use crate::dom::bindings::codegen::Bindings::NavigatorBinding::NavigatorMethods;
use crate::dom::bindings::codegen::Bindings::VRBinding::VRBinding::VRMethods;
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::{DomRoot, MutNullableDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::bluetooth::Bluetooth;
use crate::dom::gamepadlist::GamepadList;
use crate::dom::mimetypearray::MimeTypeArray;
use crate::dom::navigatorinfo;
use crate::dom::permissions::Permissions;
use crate::dom::pluginarray::PluginArray;
use crate::dom::promise::Promise;
use crate::dom::serviceworkercontainer::ServiceWorkerContainer;
use crate::dom::vr::VR;
use crate::dom::window::Window;
use dom_struct::dom_struct;
use std::rc::Rc;

#[dom_struct]
pub struct Navigator {
    reflector_: Reflector,
    bluetooth: MutNullableDom<Bluetooth>,
    plugins: MutNullableDom<PluginArray>,
    mime_types: MutNullableDom<MimeTypeArray>,
    service_worker: MutNullableDom<ServiceWorkerContainer>,
    vr: MutNullableDom<VR>,
    gamepads: MutNullableDom<GamepadList>,
    permissions: MutNullableDom<Permissions>,
}

impl Navigator {
    fn new_inherited() -> Navigator {
        Navigator {
            reflector_: Reflector::new(),
            bluetooth: Default::default(),
            plugins: Default::default(),
            mime_types: Default::default(),
            service_worker: Default::default(),
            vr: Default::default(),
            gamepads: Default::default(),
            permissions: Default::default(),
        }
    }

    pub fn new(window: &Window) -> DomRoot<Navigator> {
        reflect_dom_object(
            Box::new(Navigator::new_inherited()),
            window,
            NavigatorBinding::Wrap,
        )
    }
}

impl NavigatorMethods for Navigator {
    // https://html.spec.whatwg.org/multipage/#dom-navigator-product
    fn Product(&self) -> DOMString {
        navigatorinfo::Product()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-taintenabled
    fn TaintEnabled(&self) -> bool {
        navigatorinfo::TaintEnabled()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-appname
    fn AppName(&self) -> DOMString {
        navigatorinfo::AppName()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-appcodename
    fn AppCodeName(&self) -> DOMString {
        navigatorinfo::AppCodeName()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-platform
    fn Platform(&self) -> DOMString {
        navigatorinfo::Platform()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-useragent
    fn UserAgent(&self) -> DOMString {
        navigatorinfo::UserAgent()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-appversion
    fn AppVersion(&self) -> DOMString {
        navigatorinfo::AppVersion()
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-navigator-bluetooth
    fn Bluetooth(&self) -> DomRoot<Bluetooth> {
        self.bluetooth.or_init(|| Bluetooth::new(&self.global()))
    }

    // https://html.spec.whatwg.org/multipage/#navigatorlanguage
    fn Language(&self) -> DOMString {
        navigatorinfo::Language()
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-plugins
    fn Plugins(&self) -> DomRoot<PluginArray> {
        self.plugins.or_init(|| PluginArray::new(&self.global()))
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-mimetypes
    fn MimeTypes(&self) -> DomRoot<MimeTypeArray> {
        self.mime_types
            .or_init(|| MimeTypeArray::new(&self.global()))
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-javaenabled
    fn JavaEnabled(&self) -> bool {
        false
    }

    // https://w3c.github.io/ServiceWorker/#navigator-service-worker-attribute
    fn ServiceWorker(&self) -> DomRoot<ServiceWorkerContainer> {
        self.service_worker
            .or_init(|| ServiceWorkerContainer::new(&self.global()))
    }

    // https://html.spec.whatwg.org/multipage/#dom-navigator-cookieenabled
    fn CookieEnabled(&self) -> bool {
        true
    }

    // https://www.w3.org/TR/gamepad/#navigator-interface-extension
    fn GetGamepads(&self) -> DomRoot<GamepadList> {
        let root = self
            .gamepads
            .or_init(|| GamepadList::new(&self.global(), &[]));

        let vr_gamepads = self.Vr().get_gamepads();
        root.add_if_not_exists(&vr_gamepads);
        // TODO: Add not VR related gamepads
        root
    }
    // https://w3c.github.io/permissions/#navigator-and-workernavigator-extension
    fn Permissions(&self) -> DomRoot<Permissions> {
        self.permissions
            .or_init(|| Permissions::new(&self.global()))
    }

    // https://w3c.github.io/webvr/spec/1.1/#navigator-getvrdisplays-attribute
    #[allow(unrooted_must_root)]
    fn GetVRDisplays(&self) -> Rc<Promise> {
        self.Vr().GetDisplays()
    }
}

impl Navigator {
    pub fn Vr(&self) -> DomRoot<VR> {
        self.vr.or_init(|| VR::new(&self.global()))
    }
}
