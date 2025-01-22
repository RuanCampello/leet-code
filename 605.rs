pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut flowerbed = flowerbed;
    let mut count = 0;

    (0..flowerbed.len()).for_each(|i| {
        if flowerbed[i] == 0 {
            let left_is_empty = i == 0 || flowerbed[i - 1] == 0;
            let right_is_empty = i == flowerbed.len() - 1 || flowerbed[i + 1] == 0;

            if left_is_empty && right_is_empty {
                count += 1;
                flowerbed[i] = 1;
            }
        }
    });

    count >= n
}


fn main() {
    assert!(can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    assert!(can_place_flowers(vec![1, 0, 0, 0, 0, 0, 1], 2));
    assert!(!can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    assert!(!can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2));
}