use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

/// 第一种实现：手动遍历
fn find_difference<T: PartialEq + Clone>(vec1: &[T], vec2: &[T]) -> Vec<T> {
    let mut only_in_vec1 = Vec::new();
    let mut only_in_vec2 = Vec::new();

    for item1 in vec1 {
        if !vec2.iter().any(|item2| item1 == item2) {
            only_in_vec1.push(item1.clone());
        }
    }

    for item2 in vec2 {
        if !vec1.iter().any(|item1| item2 == item1) {
            only_in_vec2.push(item2.clone());
        }
    }

    [only_in_vec1, only_in_vec2].concat()
}

/// 双向标记法实现
fn find_difference_with_mark<T: PartialEq + Clone>(vec1: &[T], vec2: &[T]) -> Vec<T> {
    let mut matched_in_vec2 = vec![false; vec2.len()];

    let mut only_in_vec1 = Vec::new();
    for item1 in vec1 {
        let mut found = false;
        for (j, item2) in vec2.iter().enumerate() {
            if !matched_in_vec2[j] && item1 == item2 {
                matched_in_vec2[j] = true;
                found = true;
                break;
            }
        }
        if !found {
            only_in_vec1.push(item1.clone());
        }
    }

    let only_in_vec2: Vec<T> = vec2
        .iter()
        .zip(&matched_in_vec2)
        .filter(|(_, &matched)| !matched)
        .map(|(item, _)| item.clone())
        .collect();

    [only_in_vec1, only_in_vec2].concat()
}

/// 生成随机 Vec
fn generate_random_vec(size: usize, range: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..range)).collect()
}

/// 基准测试函数
fn bench_find_difference(c: &mut Criterion) {
    let vec1 = generate_random_vec(10_000, 20_000);
    let vec2 = generate_random_vec(10_000, 20_000);

    // 测试第一种方案
    c.bench_function("find_difference", |b| {
        b.iter(|| {
            let _ = find_difference(black_box(&vec1), black_box(&vec2));
        });
    });

    // 测试双向标记法
    c.bench_function("find_difference_with_mark", |b| {
        b.iter(|| {
            let _ = find_difference_with_mark(black_box(&vec1), black_box(&vec2));
        });
    });

    // 测试新方案
    c.bench_function("find_difference_new", |b| {
        b.iter(|| {
            let _ = find_difference_new(black_box(vec1.clone()), black_box(vec2.clone()));
        });
    });
}

fn find_difference_new<T: PartialEq + Clone>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    // 确保 vec1 是较小的那个，vec2 是较大的那个
    let (mut a, mut b) = if vec1.len() <= vec2.len() {
        (vec1, vec2)
    } else {
        (vec2, vec1)
    };

    // 对 a 去重
    a.dedup();

    // 给 vec1 里面的每个元素标记是否在 vec2 里出现过
    let mut matched_in_vec2 = vec![false; a.len()];

    // 查找 vec1 中的元素是否在 vec2 中，并标记已匹配的元素
    for (i, ia) in a.iter().enumerate() {
        if let Some(pos) = b.iter().position(|x| x == ia) {
            matched_in_vec2[i] = true;
            // 移除 b 中第一个匹配的元素
            b.remove(pos);
        }
    }

    // 移除 vec1 中标记为 true 的元素
    let filtered_a: Vec<_> = a
        .into_iter()
        .enumerate()
        .filter_map(|(i, item)| if matched_in_vec2[i] { None } else { Some(item) })
        .collect();

    b.dedup();

    // 合并并返回去重后的结果
    let mut result = filtered_a;
    result.extend(b);
    result
}

criterion_group!(benches, bench_find_difference);
criterion_main!(benches);
