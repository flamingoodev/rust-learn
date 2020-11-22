// use crate::routes::role::pub_print_role;

// public function
pub fn pub_print_user() {
    println!("Public function -> pri_print_user()");
}

// private function
fn pri_print_user() {
    println!("Private function -> pri_print_user()");
}

// called by super
pub fn called_by_super() {
    print!("Called by super -> super::role::pub_print_role()");
    super::role::pub_print_role();
}

// called by absolutely path
pub fn called_by_absolutely_path() {
    print!("Called by absolutely path -> crate::routes::role::pub_print_role()");
    crate::routes::role::pub_print_role();
}
