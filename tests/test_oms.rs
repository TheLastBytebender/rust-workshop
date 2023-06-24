
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Order {
	id: String,
	price: f64,
	qty: f64,
	position_idx: u8,
	created_time: i32,
	updated_time: i32,
}

enum OrderPosition {
	BuySide(Order),
	SellSide(Order),
	BuySideId(String),
	SellSideId(String)
}

enum OrderStatus {
	Pending(OrderPosition),
	Active(OrderPosition),
	Cancelled(OrderPosition)
}

#[derive(Debug, PartialEq)]
struct Oms {
	sell_side_orders_active: HashMap<String, Order>,
	sell_side_orders_pending: HashMap<String, Order>,
	buy_side_orders_active: HashMap<String, Order>,
	buy_side_orders_pending: HashMap<String, Order>
}

impl Oms {

	pub fn new() -> Oms {
		Oms { 
			sell_side_orders_active: HashMap::new(),
			sell_side_orders_pending: HashMap::new(),
			buy_side_orders_active: HashMap::new(),
			buy_side_orders_pending: HashMap::new()
		}
	}

	pub fn add_order (&mut self, order: OrderStatus) {
		match order {
			OrderStatus::Active(order_position) => {
				match order_position {
					OrderPosition::BuySide(order) => {
						self.buy_side_orders_active
							.insert(order.id.clone(), order);
					}

					OrderPosition::SellSide(order) => {
						self.sell_side_orders_active
							.insert(order.id.clone(), order);
					}

					_ => todo!()
				}
			}

			OrderStatus::Pending(order_position) => {
				match order_position {
					OrderPosition::BuySide(order) => {
						self.buy_side_orders_pending
							.insert(order.id.clone(), order);
					}

					OrderPosition::SellSide(order) => {
						self.sell_side_orders_pending
							.insert(order.id.clone(), order);
					}

					_ => todo!()
				}
			}

			OrderStatus::Cancelled(_) => todo!()
		}
	}

	pub fn get_order(&self, order: OrderStatus) -> &Order {
		match order {
			OrderStatus::Active(order_position) => {
				match order_position {
					OrderPosition::BuySideId(order_id) => {
						let value = self.buy_side_orders_active
							.get(&order_id)
							.expect("Failed to get Active buy side order");

						value
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_oms() {
    	let oms = Oms::new();

    	let oms_dummy = Oms {
    		sell_side_orders_active: HashMap::new(),
			sell_side_orders_pending: HashMap::new(),
			buy_side_orders_active: HashMap::new(),
			buy_side_orders_pending: HashMap::new()
    	};

    	assert_eq!(oms_dummy, oms)
    }

    #[test]
    fn test_add_order_oms() {
    	let mut oms = Oms::new();

    	let buy_order_active = OrderStatus::Active (
    		OrderPosition::BuySide (
    			Order {
    				id: String::from("1234"),
    				price: 12.5,
    				qty: 2.5,
    				position_idx: 1,
    				created_time: 1_000_000,
    				updated_time: 1_000_100
    			}
    		)
    	);

    	let sell_order_active = OrderStatus::Active (
    		OrderPosition::SellSide (
    			Order {
    				id: String::from("12345"),
    				price: 12.5,
    				qty: 2.5,
    				position_idx: 2,
    				created_time: 1_000_000,
    				updated_time: 1_000_100
    			}
    		)
    	);

    	oms.add_order(buy_order_active);
    	oms.add_order(sell_order_active);

    	assert_eq!(oms.buy_side_orders_active.len(), 1);
    	assert_eq!(oms.sell_side_orders_active.len(), 1);
    }
}