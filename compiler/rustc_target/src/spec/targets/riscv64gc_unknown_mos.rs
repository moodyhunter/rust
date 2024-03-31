use crate::spec::{Target, base};

pub(crate) fn target() -> Target {
    let mut baseopts = base::mos::opts();
    baseopts.linker = Some("riscv64-mos-gcc".into());
    baseopts.cpu = "generic-rv64".into();
    baseopts.features = "+m,+a,+f,+d,+c".into();
    baseopts.llvm_abiname = "lp64d".into();
    baseopts.max_atomic_width = Some(64);
    baseopts.code_model = Some(crate::spec::CodeModel::Medium);

    Target {
        llvm_target: "riscv64-unknown-mos".into(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        pointer_width: 64,
        data_layout: "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128".into(),
        arch: "riscv64".into(),
        options: baseopts,
    }
}
