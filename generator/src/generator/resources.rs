use super::{CreateInfo, ResourceInfo};

pub(super) fn for_extension(extension: &str) -> &'static [ResourceInfo<'static>] {
    match extension {
        "xproto" => &XPROTO_RESOURCES[..],
        _ => &[],
    }
}

const XPROTO_RESOURCES: [ResourceInfo<'static>; 6] = [
    ResourceInfo {
        resource_name: "Pixmap",
        create_requests: [
            Some(CreateInfo {
                request_name: "CreatePixmap",
                created_argument: "pid",
            }),
            None,
        ],
        free_request: "FreePixmap",
    },
    ResourceInfo {
        resource_name: "Window",
        create_requests: [
            Some(CreateInfo {
                request_name: "CreateWindow",
                created_argument: "wid",
            }),
            None,
        ],
        free_request: "DestroyWindow",
    },
    ResourceInfo {
        resource_name: "Font",
        create_requests: [
            Some(CreateInfo {
                request_name: "OpenFont",
                created_argument: "fid",
            }),
            None,
        ],
        free_request: "CloseFont",
    },
    ResourceInfo {
        resource_name: "Gcontext",
        create_requests: [
            Some(CreateInfo {
                request_name: "CreateGC",
                created_argument: "cid",
            }),
            None,
        ],
        free_request: "FreeGC",
    },
    ResourceInfo {
        resource_name: "Colormap",
        create_requests: [
            Some(CreateInfo {
                request_name: "CreateColormap",
                created_argument: "mid",
            }),
            None,
        ],
        free_request: "FreeColormap",
    },
    ResourceInfo {
        resource_name: "Cursor",
        create_requests: [
            Some(CreateInfo {
                request_name: "CreateCursor",
                created_argument: "cid",
            }),
            Some(CreateInfo {
                request_name: "CreateGlyphCursor",
                created_argument: "cid",
            }),
        ],
        free_request: "FreeCursor",
    },
];
