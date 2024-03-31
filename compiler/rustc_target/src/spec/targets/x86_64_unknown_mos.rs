use crate::spec::{Target, base};

pub(crate) fn target() -> Target {
    let mut baseopts = base::mos::opts();
    baseopts.linker = Some("x86_64-mos-gcc".into());

    Target {
        llvm_target: "x86_64-unknown-mos".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("x86-64 MOS".into()),
            tier: None,
            host_tools: None,
            std: Some(true),
        },
        pointer_width: 64,
        data_layout:
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: "x86_64".into(),
        options: baseopts,
    }
}
