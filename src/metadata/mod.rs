mod resolver;
mod types;

pub use resolver::MetadataResolver;
pub use types::{
    ConflictedTarget, ContextAnchor, MetadataDocument, MetadataEntry, MetadataReconcileRequest,
    MetadataReconcileResult, MetadataTarget, TargetResolution, TargetResolutionKind,
    UnresolvedTarget,
};
