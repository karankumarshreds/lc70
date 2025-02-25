mod two_pointers;
mod sliding_window;

fn main() {
    two_pointers::move_zeros::run();
    two_pointers::is_subsequence::run();
    two_pointers::most_water::run();
    two_pointers::max_k_sum_pairs::run();

    sliding_window::max_sub_array_avg::run();
}
