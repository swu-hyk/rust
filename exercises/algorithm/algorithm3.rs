/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where
    T: Ord, // 限制 T 必须实现 Ord trait，以支持比较操作
{
    let len = array.len();
    if len <= 1 {
        return; // 如果数组长度为 0 或 1，则无需排序
    }

    // 插入排序主逻辑
    for i in 1..len {
        let mut j = i;
        // 将当前元素与已排序部分的元素逐一比较，直到找到正确的位置
        while j > 0 && array[j] < array[j - 1] {
            // 交换相邻元素
            array.swap(j, j - 1);
            j -= 1; // 继续向前比较
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}