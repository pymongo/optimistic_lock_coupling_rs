use criterion::{criterion_group, BenchmarkId, Criterion, Fun, ParameterizedBenchmark};

use super::ListMutex;
use super::ListOLock;

fn heavy_read_list_mutex(i: i32) {
    static mut lock: Option<ListMutex> = None;
    unsafe { lock = Some(ListMutex::new()) };
    let write_fn = move || unsafe {
        for _i in 0..i {
            // std::thread::sleep_ms(10);
            lock.as_ref().unwrap().set2();
            // std::thread::sleep_ms(12);
            lock.as_ref().unwrap().set3();
        }
    };
    let read_fn1 = move || unsafe {
        for _i in 0..i {
            // std::thread::sleep_ms(8);
            lock.as_ref().unwrap().get_all();
        }
    };
    let read_fn2 = move || unsafe {
        for _i in 0..i {
            // std::thread::sleep_ms(8);
            lock.as_ref().unwrap().get2();
        }
    };
    use std::thread::spawn;
    let thread1 = spawn(write_fn);
    let thread2 = spawn(write_fn);
    let thread3 = spawn(read_fn2);
    let thread4 = spawn(read_fn1);
    let thread5 = spawn(read_fn2);
    let _ = thread1.join();
    let _ = thread2.join();
    let _ = thread3.join();
    let _ = thread4.join();
    let _ = thread5.join();
    unsafe { assert_eq!(lock.as_ref().unwrap().get2(), 2 * i) }
}
fn heavy_read_list_optimistic_lock_coupling(i: i32) {
    static mut lock: Option<ListOLock> = None;
    unsafe { lock = Some(ListOLock::new()) };
    let write_fn = move || unsafe {
        for _i in 0..i {
            // std::thread::sleep_ms(10);
            lock.as_ref().unwrap().set2();
            // std::thread::sleep_ms(12);
            lock.as_ref().unwrap().set3();
        }
    };
    let read_fn1 = move || unsafe {
        for _i in 0..i {
            // std::thread::sleep_ms(8);
            lock.as_ref().unwrap().get_all();
        }
    };
    let read_fn2 = move || unsafe {
        for _i in 0..i {
            // std::thread::sleep_ms(8);
            lock.as_ref().unwrap().get2();
        }
    };
    use std::thread::spawn;
    let thread1 = spawn(write_fn);
    let thread2 = spawn(write_fn);
    let thread3 = spawn(read_fn2);
    let thread4 = spawn(read_fn1);
    let thread5 = spawn(read_fn2);
    let _1 = thread1.join().unwrap();
    let _2 = thread2.join().unwrap();
    let _3 = thread3.join().unwrap();
    let _4 = thread4.join().unwrap();
    let _5 = thread5.join().unwrap();
    unsafe { assert_eq!(lock.as_ref().unwrap().get2(), 2 * i) }
}

fn lock_heavy_read_list(c: &mut Criterion) {
    let mx = Fun::new("Mutex", |b, i| b.iter(|| heavy_read_list_mutex(*i)));
    let ol = Fun::new("OptimisticLockCoupling", |b, i| {
        b.iter(|| heavy_read_list_optimistic_lock_coupling(*i))
    });

    let functions = vec![ol, mx];

    c.bench_functions("Lock Heavy Read List Compare", functions, 1_000_000);
}

criterion_group!(name = lock_heavy_read; config = Criterion::default().sample_size(1_000); targets = lock_heavy_read_list);
