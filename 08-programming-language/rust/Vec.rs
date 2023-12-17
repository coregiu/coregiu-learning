pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let count = nums.len();
        for i in 0..count {
            for j in i + 1..count {
                if &nums[i] + &nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![0, 1]
    }
}
fn main() {
    // vector初始化
    // let vecs = vec![3, 2, 4];
    let mut vecs: Vec<i32> = Vec::new();

    // 添加元素
    vecs.push(3);
    vecs.push(2);
    vecs.push(4);

    // 获取向量长度
    println!("vecs len = {}", vecs.len());

    // 弹出元素，后进先出
    let pop_val = vecs.pop().unwrap();
    println!("pop_val = {}, len = {}", pop_val, vecs.len());

    // 在某位置插入元素
    vecs.insert(1, 5);
    println!("-----insert element-----");
    for i in 0..vecs.len() {
        println!("{}-{}", i, &vecs[i]);
    }

    // 删除元素
    println!("-----remove element-----");
    vecs.remove(1);
    for i in 0..vecs.len() {
        println!("{}-{}", i, &vecs[i]);
    }

    println!("-----invoke sum function-----");
    let result = Solution::two_sum(vecs, 5);
    println!("{},{}", &result[0], &result[1]);

    // 按范围删除元素
    println!("-----retain element-----");
    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.retain(|&x| x % 2 == 0);
    for i in &my_vec {
        println!("{i}");
    }

    println!("-----drain element-----");
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let new_vec: Vec<i32> = my_vec.drain(1..3).collect();
    for i in &new_vec {
        println!("{i}");
    }

    println!("-----update element-----");
    // 修改元素
    let mut my_vec = vec![1, 2, 3, 4, 5];
    *&mut my_vec[2] = 100;
    println!("{}", &my_vec[2]);
}
