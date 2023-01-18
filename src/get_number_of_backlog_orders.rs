use std::{
    cmp::{self, Ordering, Reverse},
    collections::BinaryHeap,
};

const NUM: i32 = 10_i32.pow(9) + 7;

pub struct Solution {}

#[derive(PartialEq, Eq, Hash)]
struct BuyOrder {
    price: i32,
    amount: i32,
}

impl BuyOrder {
    fn new(data: Vec<i32>) -> BuyOrder {
        BuyOrder {
            price: data[0],
            amount: data[1],
        }
    }
}

impl PartialOrd for BuyOrder{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl Ord for BuyOrder {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.price.cmp(&other.price)
    }
}

#[derive(PartialEq, Eq, Hash)]
struct SellOrder {
    price: i32,
    amount: i32,
}

impl SellOrder {
    fn new(data: Vec<i32>) -> SellOrder {
        SellOrder {
            price: data[0],
            amount: data[1],
        }
    }
}

impl PartialOrd for SellOrder{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.price.partial_cmp(&self.price)
    }
}

impl Ord for SellOrder {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.price.cmp(&other.price) == cmp::Ordering::Less {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

// fn print_heap(sell_heap: &BinaryHeap<SellOrder>, buy_heap: &BinaryHeap<BuyOrder>) {
//     println!("sell heap",);
//     for i in sell_heap {
//         println!("{}, {}", i.price, i.amount);
//     }

//     println!("buy heap",);
//     for i in buy_heap {
//         println!("{}, {}", i.price, i.amount);
//     }
// }

impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        //小顶堆
        let mut sell_heap: BinaryHeap<SellOrder> = BinaryHeap::new();
        //大顶堆
        let mut buy_heap: BinaryHeap<BuyOrder> = BinaryHeap::new();
        let mut result = 0;

        'order: for order in orders {
            // print_heap(&sell_heap, &buy_heap);
            if order[2] == 0 {
                let mut buy_order = BuyOrder::new(order);
                while sell_heap.is_empty() == false && buy_order.amount > 0 {
                    let sell_order_price = sell_heap.peek().unwrap().price;
                    let sell_order_amount = sell_heap.peek().unwrap().amount;                    
                    if sell_order_price <= buy_order.price {
                        if sell_order_amount <= buy_order.amount {
                            buy_order.amount -= sell_order_amount;
                            sell_heap.pop();
                        } else {
                            sell_heap.peek_mut().unwrap().amount -= buy_order.amount;
                            continue 'order;
                        }
                    } else {
                        break;
                    }
                }
                if buy_order.amount > 0 {
                    buy_heap.push(buy_order);
                }
            } else {
                let mut sell_order = SellOrder::new(order);
                while buy_heap.is_empty() == false && sell_order.amount > 0 {
                    let buy_order_price = buy_heap.peek().unwrap().price;
                    let buy_order_amount = buy_heap.peek().unwrap().amount;                    
                    if sell_order.price <= buy_order_price {
                        if buy_order_amount <= sell_order.amount {
                            sell_order.amount -= buy_order_amount;
                            buy_heap.pop();
                        } else {
                            buy_heap.peek_mut().unwrap().amount -= sell_order.amount;
                            continue 'order;
                        }
                    } else {
                        break;
                    }
                }
                if sell_order.amount > 0 {
                    sell_heap.push(sell_order);
                }
            }
        }

        for i in sell_heap {
            result = (result + i.amount) % NUM;
        }

        for i in buy_heap {
            result = (result + i.amount) % NUM
        }
        result
    }
}
