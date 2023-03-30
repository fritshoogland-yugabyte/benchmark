use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::io::prelude::*;
use std::time::Duration;

fn remove_and_create_with_size(
	name: &str,
	size: usize,
) -> fs::File
{
	fs::remove_file(&name).unwrap_or_else(|_| {});
	let mut file = fs::OpenOptions::new().create(true).write(true).open(&name).unwrap();
	let data = vec![0_u8; size]; // create vector for the size of the file
	file.write_all(&data).expect("Error during write_all()");
	file
}
fn write_no_sync(
	file: &mut fs::File,
	payload: &Vec<u8>,
	file_size: usize,
)
{
        // reset file pointer back to the beginning of the file if it would exceed file size
	if file.stream_position().unwrap() + payload.len() as u64 > file_size.try_into().unwrap()
        {
		file.rewind().unwrap();
        }
	file.write_all(&payload).unwrap();
}
fn write_fsync(
	file: &mut fs::File,
	payload: &Vec<u8>,
	file_size: usize,
)
{
        // reset file pointer back to the beginning of the file if it would exceed file size
	if file.stream_position().unwrap() + payload.len() as u64 > file_size.try_into().unwrap()
        {
		file.rewind().unwrap();
        }
	file.write_all(&payload).unwrap();
	file.sync_all().unwrap(); // fsync()
}
fn write_fdatasync(
	file: &mut fs::File,
	payload: &Vec<u8>,
	file_size: usize,
)
{
        // reset file pointer back to the beginning of the file if it would exceed file size
	if file.stream_position().unwrap() + payload.len() as u64 > file_size.try_into().unwrap()
        {
		file.rewind().unwrap();
        }
	file.write_all(&payload).unwrap();
	file.sync_data().unwrap(); // fdatasync()
}

fn benchmark_fsync(c: &mut Criterion) 
{
    let file_size = 67108864; // 64M
    //let write_size = 1024;    // 1k
    let write_size = 24576;   // 24k

    let payload = vec![0_u8; write_size];
    let mut file = remove_and_create_with_size("/tmp/test", file_size);

    let mut group = c.benchmark_group("Fsync");

    file.sync_all().unwrap();
    file.rewind().unwrap();
    group.bench_function("no sync", |b| b.iter(|| write_no_sync(&mut file, &payload, file_size)));

    file.sync_all().unwrap();
    file.rewind().unwrap();
    group.bench_function("fsync", |b| b.iter(|| write_fsync(&mut file, &payload, file_size)));

    file.sync_all().unwrap();
    file.rewind().unwrap();
    group.bench_function("fdatasync", |b| b.iter(|| write_fdatasync(&mut file, &payload, file_size)));
    
    group.finish();
}

criterion_group!(
	name = benchmarks;
	config = Criterion::default().measurement_time(Duration::from_secs(10));
	targets = benchmark_fsync
);
criterion_main!(benchmarks);
