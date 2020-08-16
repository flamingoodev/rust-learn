use rust_learn::routes;
use rust_learn::routes::role;
// use rust_learn::games::guess_number;
use rust_learn::routes::user;

fn main() {
    role::pub_print_role();
    routes::role::pub_print_role();
    // guess_number::start();
    user::called_by_super();
    user::called_by_absolutely_path();
}
