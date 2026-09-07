use criterion::{Criterion, criterion_group, criterion_main};
use grammers_stringsession::grammers_session::storages::MemorySession;
use std::hint::black_box;

const STRING_SESSION: &str = "AAAApBMAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAYAEMDAwMTAyMDMwNDA1MDYwNzA4MDkwYTBiMGMwZDBlMGYxMDExMTIxMzE0MTUxNjE3MTgxOTFhMWIxYzFkMWUxZjIwMjEyMjIzMjQyNTI2MjcyODI5MmEyYjJjMmQyZTJmMzAzMTMyMzMzNDM1MzYzNzM4MzkzYTNiM2MzZDNlM2Y0MDQxNDI0MzQ0NDU0NjQ3NDg0OTRhNGI0YzRkNGU0ZjUwNTE1MjUzNTQ1NTU2NTc1ODU5NWE1YjVjNWQ1ZTVmNjA2MTYyNjM2NDY1NjY2NzY4Njk2YTZiNmM2ZDZlNmY3MDcxNzI3Mzc0NzU3Njc3Nzg3OTdhN2I3YzdkN2U3ZjgwODE4MjgzODQ4NTg2ODc4ODg5OGE4YjhjOGQ4ZThmOTA5MTkyOTM5NDk1OTY5Nzk4OTk5YTliOWM5ZDllOWZhMGExYTJhM2E0YTVhNmE3YThhOWFhYWJhY2FkYWVhZmIwYjFiMmIzYjRiNWI2YjdiOGI5YmFiYmJjYmRiZWJmYzBjMWMyYzNjNGM1YzZjN2M4YzljYWNiY2NjZGNlY2ZkMGQxZDJkM2Q0ZDVkNmQ3ZDhkOWRhZGJkY2RkZGVkZmUwZTFlMmUzZTRlNWU2ZTdlOGU5ZWFlYmVjZWRlZWVmZjBmMWYyZjNmNGY1ZjZmN2Y4ZjlmYWZiZmNmZGZlZmY=";

fn criterion_benchmark(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("Export MemorySession", |b| {
        let session = MemorySession::default();
        b.iter(|| grammers_stringsession::export(black_box(&session)))
    });
    c.bench_function("Import MemorySession", |b| {
        let session = MemorySession::default();
        b.to_async(&runtime).iter(|| {
            grammers_stringsession::restore(black_box(&session), black_box(STRING_SESSION))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
