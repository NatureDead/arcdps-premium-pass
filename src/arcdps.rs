use crate::{exports::*, main, release};

arcdps::arcdps_export! {
    name: "Premium Pass",
    sig: 0x2_0804,
    init: main,
    release: release,
    imgui: imgui,
}
