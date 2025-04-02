/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T> {
    // 使用两个队列来模拟栈的行为
    q1: Queue<T>,
    q2: Queue<T>
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            // 初始化两个队列
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new()
        }
    }

    pub fn push(&mut self, elem: T) {
        // 将元素压入 q1 队列
        self.q1.enqueue(elem);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        // 如果 q1 为空，返回错误信息
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        // 将 q1 中的所有元素（除了最后一个）移动到 q2
        while self.q1.size() > 1 {
            if let Ok(value) = self.q1.dequeue() {
                self.q2.enqueue(value);
            }
        }

        // q1 中剩下的最后一个元素即为栈顶元素，将其弹出并返回
        let top = self.q1.dequeue().unwrap();

        // 交换 q1 和 q2 的角色，以便下次操作
        std::mem::swap(&mut self.q1, &mut self.q2);

        Ok(top)
    }

    pub fn is_empty(&self) -> bool {
        // 栈为空当且仅当 q1 为空
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}