pub fn part1(list: Vec<String>) -> u32 {
    let mut sum = 0;

    for item in list {
        sum += calibration_value(&item)
    }

    return sum;
}

pub fn part2(list: Vec<String>) -> u32 {
    let mut sum = 0;

    for item in list {
        // do some cringe stuff here:
        //  (o)n[e]
        //  (t)w[o]
        //  (t)hre[e]
        //  four
        //  fiv[e]
        //  six
        //  seve[n]
        //  (e)igh[t]
        //  (n)in[e]
        let mut item_fix = item.to_owned();
        item_fix = item_fix.replace("one", "o1e");
        item_fix = item_fix.replace("two", "t2o");
        item_fix = item_fix.replace("three", "t3e");
        item_fix = item_fix.replace("four", "4");
        item_fix = item_fix.replace("five", "5e");
        item_fix = item_fix.replace("six", "6");
        item_fix = item_fix.replace("seven", "7n");
        item_fix = item_fix.replace("eight", "e8t");
        item_fix = item_fix.replace("nine", "n9e");

        sum += calibration_value(&item_fix);
    }

    return sum;
}

// calculate numeric string value
fn calibration_value(string: &str) -> u32 {
    let mut num_chars: Vec<u32> = Vec::new();

    // extract nums from str
    for i in string.chars() {
        if i.is_numeric() {
            num_chars.push(i.to_digit(10).unwrap())
        }
    }

    // get first and last nums from vec
    let num_first = num_chars.first().unwrap();
    let num_last = num_chars.last().unwrap_or(num_first);

    num_first * 10 + num_last
}
