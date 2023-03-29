use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::io::prelude::*;

fn remove_and_create_with_size(
	name: &str,
	size: usize,
) -> fs::File
{
	fs::remove_file(&name).unwrap_or_else(|_| {});
	let mut file = fs::OpenOptions::new().create(true).write(true).open(&name).unwrap();
	let data = vec![0_u8; size];
	file.write_all(&data).expect("Error during write_all()");
	file
}
fn write_no_sync(
	file: &mut fs::File,
	payload: &Vec<u8>,
)
{
	file.write_all(&payload).unwrap();
}
fn write_fsync(
	file: &mut fs::File,
	payload: &Vec<u8>,
)
{
	file.write_all(&payload).unwrap();
	file.sync_all().unwrap();
}
fn write_fdatasync(
	file: &mut fs::File,
	payload: &Vec<u8>,
)
{
	file.write_all(&payload).unwrap();
	file.sync_data().unwrap();
}

fn benchmark_fsync(c: &mut Criterion) 
{
    let file_size = 67108864; // 64M
    //let write_size = 1024;    // 1k
    let write_size = 24576;   

    let payload = vec![0_u8; write_size];
    let mut file = remove_and_create_with_size("/tmp/test", file_size);

    let mut group = c.benchmark_group("Fsync");

    file.sync_all().unwrap();
    file.rewind().unwrap();
    group.bench_function("no sync", |b| b.iter(|| write_no_sync(&mut file, &payload)));

    file.sync_all().unwrap();
    file.rewind().unwrap();
    group.bench_function("fsync", |b| b.iter(|| write_fsync(&mut file, &payload)));

    file.sync_all().unwrap();
    file.rewind().unwrap();
    group.bench_function("fdatasync", |b| b.iter(|| write_fdatasync(&mut file, &payload)));
    
    group.finish();
}

criterion_group!(benchmarks, benchmark_fsync);
criterion_main!(benchmarks);
