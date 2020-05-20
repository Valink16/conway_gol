use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand;

use gol::{grid, neigh, render};

const W: usize = 100;
const H: usize = 100;

pub fn cell_repr(c: &mut Criterion) {
    let mut g = grid::Grid::new(W, H);
    c.bench_function("bench_grid_borrowed_n_buffer", |b| { // This represents the cells as 2 different arrays, with one as the current and one as the future
        b.iter(|| {
            let mut n = [0 as usize; 8];
            for i in 0..g.cells.len() as isize {
                match g.get_neighbors(i, &mut n) {
                    Some(_) => {
                        for _n in n.iter() {
                            g.cells[*_n].0 = g.cells[*_n].1;
                        }
                    },
                    None => ()
                };
            }
        })
    });

    c.bench_function("bench_grid_update_cells", |b| { 
        b.iter(|| {
            g.update_cells();
        })
    });
}

pub fn rand_gen_bench(c: &mut Criterion) {
    let mut g = grid::Grid::new(W, H);

    c.bench_function("bench_randomize_grid", |b| { 
        b.iter(|| {
            for i in 0..g.cells.len() as isize {
                match neigh::NeighborIter::new(&g.n_ops, i, g.w, g.h) {
                    Some(n) => {
                        for _n in n {
                            g.cells[_n].0 = g.cells[_n].1;
                        }
                    },
                    None => ()
                };
            }
        })
    });
}

pub fn vbuffer(c: &mut Criterion) {
    let mut g = grid::Grid::new(W, H);
    let mut r = render::Renderer::new(
        "Renderer test", 600, 400,
        g
    );


    c.bench_function("update_vbuffer", |b| { 
        b.iter(|| {
            r.update_vbuffer();
        })
    });
}

criterion_group!(benches, cell_repr);
criterion_main!(benches);