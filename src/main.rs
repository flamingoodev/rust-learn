// use rust_learn::games::guess_number;
use rust_learn::collections::t_vector;
use rust_learn::routes::{role, user};
use rust_learn::syntax::t_enum;
use rust_learn::syntax::t_string;
use rust_learn::collections::t_map;

fn main() {
    role::pub_print_role();
    role::pub_print_role();
    // guess_number::start();
    user::called_by_super();
    user::called_by_absolutely_path();
    t_vector::vec_new();
    t_enum::t_enum();
    t_string::t_string();
    t_map::t_map();
}
