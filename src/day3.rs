pub fn part1(list: Vec<String>) -> u32 {
    let mut sum = 0;

    fn is_symbol(c: char) -> bool {
        c != '.' && !c.is_numeric()
    }

    // 2d map time
    for (i, item) in list.iter().enumerate() {
        let prev_item = list.iter().nth(if i > 0 { i - 1 } else { list.len() });
        let next_item = list.iter().nth(i + 1);

        // create some pointers
        // -1 = unset
        let mut ptr_left = -1;
        let mut ptr_right = -1;

        fn unset_ptr(ptr: i32) -> bool {
            ptr < 0
        }

        'item_loop: for (char_i, char_item) in item.chars().enumerate() {
            let next_char = item.chars().nth(char_i + 1);

            // set start of left pointer
            if unset_ptr(ptr_left) && char_item.is_numeric() {
                ptr_left = i_to_int(char_i);
            }

            // set start of right pointer
            if unset_ptr(ptr_right) && !unset_ptr(ptr_left) && char_item.is_numeric() {
                ptr_right = i_to_int(char_i);
            }

            // increment right pointer
            if char_item.is_numeric() {
                ptr_right = i_to_int(char_i);
            }

            // end of ptr capture
            if char_item.is_numeric()
                && !unset_ptr(ptr_right)
                && !unset_ptr(ptr_left)
                && !next_char.is_some_and(char::is_numeric)
            {
                // process the captured number
                {
                    let num = item
                        .chars()
                        .skip(int_to_i(ptr_left))
                        .take(int_to_i(ptr_right - ptr_left) + 1)
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();

                    // get the character before the pointer begins
                    let prev_char = if ptr_left > 0 {
                        item.chars().nth(int_to_i(ptr_left) - 1)
                    } else {
                        None
                    };

                    // get the character after the pointer ends
                    let next_char = item.chars().nth(int_to_i(ptr_right) + 1);

                    fn is_item_adjacent(item: &String, ptr_left: i32, ptr_right: i32) -> bool {
                        // get the search target's offset value relative to the true pos
                        let item_offset = if ptr_left > 0 { ptr_left - 1 } else { 0 };

                        // find position of the symbol (*)
                        fn find_symbol_pos(string: &str) -> i32 {
                            match string.find(|c: char| is_symbol(c)) {
                                Some(pos) => i_to_int(pos),
                                None => -1,
                            }
                        }

                        // slice the item string with the offset (if allowed)
                        // this is required as the find_symbol_pos only looks for the **first** match
                        let sym_pos = find_symbol_pos(&item[int_to_i(item_offset)..item.len()]);

                        // calculate relative pointer values
                        let rel_ptr_left = ptr_left - item_offset;
                        let rel_ptr_right = ptr_right - item_offset;

                        // check adjacency diagonally
                        let sym_left_diag = rel_ptr_left - sym_pos == 1;
                        let sym_right_diag = sym_pos - rel_ptr_right == 1;

                        // check normal adjacency
                        let sym_adjacent = rel_ptr_left <= sym_pos && rel_ptr_right >= sym_pos;

                        // return result
                        return sym_left_diag || sym_right_diag || sym_adjacent;
                    }

                    // check if the previous item (line) is an adjacent symbol
                    let prev_item_check = match prev_item {
                        Some(item) => is_item_adjacent(item, ptr_left, ptr_right),
                        None => false,
                    };

                    // check if the next item (line) is an adjacent symbol
                    let next_item_check = match next_item {
                        Some(item) => is_item_adjacent(item, ptr_left, ptr_right),
                        None => false,
                    };

                    // check if the current number captured is adjacent to a symbol
                    let prev_char_check = prev_char.is_some() && is_symbol(prev_char.unwrap());
                    let next_char_check = next_char.is_some() && is_symbol(next_char.unwrap());

                    let x_adjacent_check = prev_char_check || next_char_check;
                    let y_adjacent_check = next_item_check || prev_item_check;

                    // add num to the sum if the num is adjacent
                    // with a symbol in either x or y direction
                    if x_adjacent_check || y_adjacent_check {
                        sum += num;
                    }
                }

                // reset pointer and continue to the next number in the item (if any)
                ptr_left = -1;
                ptr_right = -1;
                continue 'item_loop;
            }
        }
    }

    return sum;
}

pub fn part2(list: Vec<String>) -> u32 {
    let mut sum = 0;

    fn find_number(string: &str, pos: i32) -> u32 {
        // find number by incrementing 2 pointers left and right respectively
        let mut ptr_left = pos;
        let mut found_ptr_begin = false;

        let mut ptr_right = pos;
        let mut found_ptr_end = false;

        // increment left ptr to the left until the last valid digit
        while found_ptr_begin == false {
            if ptr_left == 0 {
                break;
            }

            let c = string.chars().nth(int_to_i(ptr_left - 1));
            match c {
                Some(c) if c.is_numeric() => ptr_left -= 1,
                _ => found_ptr_begin = true,
            }
        }

        // increment right ptr to the right until the last valid digit
        while found_ptr_end == false {
            if ptr_right >= i_to_int(string.len()) {
                break;
            }

            let c = string.chars().nth(int_to_i(ptr_right + 1));
            match c {
                Some(c) if c.is_numeric() => ptr_right += 1,
                _ => found_ptr_end = true,
            }
        }

        // get num from string as slice
        return string
            .chars()
            .skip(int_to_i(ptr_left))
            .take(int_to_i(ptr_right - ptr_left) + 1)
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
    }

    // 2d map time - this time we do it backwards
    // (we count the symbols first before the nums)
    for (i, item) in list.iter().enumerate() {
        let prev_item = list.iter().nth(if i > 0 { i - 1 } else { list.len() });
        let next_item = list.iter().nth(i + 1);

        for (char_i, char_item) in item.chars().enumerate() {
            let prev_char_pos = if char_i > 0 { char_i - 1 } else { item.len() };
            let prev_char = item.chars().nth(prev_char_pos);

            let next_char_pos = char_i + 1;
            let next_char = item.chars().nth(next_char_pos);

            fn vert_lookup(item: &String, char_i: usize, result: &mut Vec<u32>) {
                // let mut result: Vec<u32> = Vec::new();

                let num_center_char = item.chars().nth(char_i);
                let mut num_center = 0;
                if num_center_char.is_some() && num_center_char.unwrap().is_numeric() {
                    let num = find_number(&item, i_to_int(char_i));
                    num_center = num;
                    result.push(num);
                }

                let num_left_pos = char_i - 1;
                let num_left_char = item.chars().nth(num_left_pos);
                if num_left_char.is_some() && num_left_char.unwrap().is_numeric() {
                    let num = find_number(&item, i_to_int(num_left_pos));
                    if num != num_center {
                        result.push(num);
                    }
                }

                let num_right_pos = char_i + 1;
                let num_right_char = item.chars().nth(num_right_pos);
                if num_right_char.is_some() && num_right_char.unwrap().is_numeric() {
                    let num = find_number(&item, i_to_int(num_right_pos));
                    if num != num_center {
                        result.push(num);
                    }
                }

                // return result;
            }

            // just look for the single symbol (*)
            // and do the relation searching
            if char_item == '*' {
                let mut result: Vec<u32> = Vec::new();

                // look up for number
                if prev_item.is_some() {
                    vert_lookup(prev_item.unwrap(), char_i, &mut result);
                }
                // look down for number
                if next_item.is_some() {
                    vert_lookup(next_item.unwrap(), char_i, &mut result);
                }

                // look left for number
                if prev_char.is_some() && prev_char.unwrap().is_numeric() {
                    let num = find_number(&item, i_to_int(prev_char_pos));
                    result.push(num);
                }
                // look right for number
                if next_char.is_some() && next_char.unwrap().is_numeric() {
                    let num = find_number(&item, i_to_int(next_char_pos));
                    result.push(num);
                }

                // only multiply result (ratio) and add to sum
                if result.len() >= 2 {
                    let mult_result = result.into_iter().reduce(|total, n| total * n).unwrap_or(0);
                    sum += mult_result;
                }
            }
        }
    }

    return sum;
}

// convert usize (pointer) to signed int
fn i_to_int(i: usize) -> i32 {
    i32::try_from(i).unwrap()
}

// convert signed int to pointer (usize)
fn int_to_i(int: i32) -> usize {
    usize::try_from(int).unwrap()
}
