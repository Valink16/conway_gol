use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gol::grid;

pub fn cell_repr(c: &mut Criterion) {
    c.bench_function("bench_grid_opt", |b| { // This represents the cells as 2 different arrays, with one as the current and one as the future
        let mut g = grid::Grid::new(1000, 1000);

        b.iter(|| {
            g.update_cells();
        })
    });

    c.bench_function("bench_double_array_naive", |b| { // This represents the cells as 2 different arrays, with one as the current and one as the future
        let mut array1 = vec![false; 1000 * 1000];
        let mut array2 = vec![false; 1000 * 1000];

        b.iter(|| {
            for i in 0..array1.len() {
                array2[i] = !array1[i];
            }
            array1.clone_from(&array2);
        })
    });

    c.bench_function("bench_single_array_naive", |b| { // This represents the cells as 2 different arrays, with one as the current and one as the future
        let mut array1 = vec![(false, false); 1000 * 1000];

        b.iter(|| {
            for i in 0..array1.len() {
                array1[i].0 = !array1[i].1;
            }
        })
    });
}

criterion_group!(benches, cell_repr);
criterion_main!(benches);