/// All the raw bindgen output.
pub mod wt_raw {
    #![allow(dead_code)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// A more typesafe interface.
pub mod wt;

pub mod proto {
    pub mod echo;
    pub mod echo_grpc;

    pub mod kv;
    pub mod kv_grpc;
}
