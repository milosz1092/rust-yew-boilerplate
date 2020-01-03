pub fn count_prime_numbers_in_range(begin: i32, end: i32) -> i32 {
    let mut how_many: i32 = 0;
    let mut pointer: i32 = begin;
    let mut flag: i32;

    while pointer < end + 1 {
        flag = 0;

        for i in 2..pointer {
            if (pointer % i) == 0 {
                flag = 1;
                break;
            }
        }

        if flag == 0 {
            how_many += 1;
        }

        pointer += 1;
    }

    how_many
}