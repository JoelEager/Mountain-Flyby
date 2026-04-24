use criterion::{Criterion, black_box, criterion_group, criterion_main};

// A mock struct for CommandBuffer since we can't easily mock Vulkan without a lot of setup
#[derive(Clone, Copy)]
struct CommandBuffer(u64);

fn bench_vec_allocation(c: &mut Criterion) {
    let draw_command_buffer = CommandBuffer(42);

    c.bench_function("vec_allocation", |b| {
        b.iter(|| {
            let command_buffers = vec![black_box(draw_command_buffer)];
            black_box(command_buffers);
        })
    });
}

fn bench_array_allocation(c: &mut Criterion) {
    let draw_command_buffer = CommandBuffer(42);

    c.bench_function("array_allocation", |b| {
        b.iter(|| {
            let command_buffers = [black_box(draw_command_buffer)];
            black_box(command_buffers);
        })
    });
}

criterion_group!(benches, bench_vec_allocation, bench_array_allocation);
criterion_main!(benches);
