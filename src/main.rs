mod linked_list;
mod graph_traversal;
mod bfs;
mod sliding_window_max_sum;
mod two_sum_sorted;
use bfs::test_bfs;
use graph_traversal::test_dfs;
use linked_list::test_linked_list;
use sliding_window_max_sum::test_sliding_window_max_sum;
use two_sum_sorted::test_two_sum_sorted;

fn main() {
    println!("Hello, world!");
    test_linked_list();
    test_dfs();
    test_bfs();
    test_sliding_window_max_sum();
    test_two_sum_sorted();
}
