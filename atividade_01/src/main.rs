fn main() {
    use restaurant::front_of_house::hosting;
    use restaurant::back_of_house;
    use restaurant::PI;

    hosting::add_to_wait_list();
    hosting::seat_at_table();

    back_of_house::take_care_trash();

    println!("The value of PI is: {}", PI);
}
