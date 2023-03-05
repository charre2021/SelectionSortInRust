fn take_size() -> usize
{
    let mut line = String::new();
    println!("Enter how long your array should be!");
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().parse::<usize>().unwrap();
}

fn create_array(sz : usize) -> Vec<i32>
{
    let mut unsorted = Vec::new();
    for _ in 0..sz
    {
        let mut line_two = String::new();
        println!("Enter a number:");
        std::io::stdin().read_line(&mut line_two).unwrap();
        unsorted.push(line_two.trim().parse::<i32>().unwrap());
    }
    return unsorted;
}

fn swap(first_num : i32, second_num : i32) -> (i32, i32)
{
    return (second_num, first_num)
}

fn main()
{
    let arr_size = take_size();
    let mut unsorted = create_array(arr_size);
    for i in 0..arr_size
    {
        let mut min_value_index = i;
        for j in i+1..arr_size
        {
            min_value_index = if unsorted[j] < unsorted[min_value_index] {j} else {min_value_index};
        }
        (unsorted[i], unsorted[min_value_index]) = swap(unsorted[i], unsorted[min_value_index])

    }
    println!("Here is your sorted vector by selection sort: ");
    for i in unsorted
    {
        println!("{} ", i)
    }
}
