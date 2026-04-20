//! High-level smoke tests for the core crate.

#[tokio::test]
async fn ingest_path_stub_counts() {
    let dir = std::env::temp_dir();
    let count = omniforge_core::data_pipeline::ingest::import_path(&dir)
        .await
        .expect("import temp dir");
    assert_eq!(count, 1);
}

#[tokio::test]
async fn inference_engine_stub_errors() {
    use omniforge_core::inference::config::InferenceParams;
    use omniforge_core::inference::engine::InferenceEngine;

    let err = InferenceEngine::complete("ping", &InferenceParams::default())
        .await
        .expect_err("engine should be unconfigured in scaffold");
    let msg = err.to_string();
    assert!(msg.contains("llama"), "unexpected error: {msg}");
}

#[test]
fn rag_constants_sane() {
    assert!(
        omniforge_core::rag::chunker::DEFAULT_CHUNK_OVERLAP
            < omniforge_core::rag::chunker::DEFAULT_CHUNK_TOKENS
    );
}
